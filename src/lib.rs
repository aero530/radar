// cSpell:disable

#[macro_use]
extern crate num_derive;

use serde::{Deserialize, Serialize};
use std::io::Read;

use bzip2::bufread::BzDecoder;
use tracing::{info, warn};

mod codes;
use codes::MessageCode;

mod message_header;
use message_header::{message_header, MessageHeader};

mod product_description;
use product_description::{product_description, ProductDescription};

mod product_symbology;
use product_symbology::{symbology, SymbologyBlock};

mod graphic_alphanumeric;
use graphic_alphanumeric::GraphicBlock;

mod tabular_alphanumeric;
use tabular_alphanumeric::TabularBlock;

mod text_header;
use text_header::{text_header, TextHeader};

mod plot;

mod error_r;
use error_r::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Radar {
    text_header: TextHeader,
    message_header: MessageHeader,
    product_description: ProductDescription,
    symbology: Option<SymbologyBlock>,
    graphic: Option<GraphicBlock>,
    tabular: Option<TabularBlock>,
}


impl Radar {
    fn parse<'a>(header_section: &'a [u8], remaining_file: &'a [u8]) -> Result<(&'a [u8], Radar), Error> {
        info!("File is {:?} bytes.", header_section.len());
        info!("Decode is {:?} bytes.", remaining_file.len());
        
        // Text header
        let (input_header, text_header) = text_header(header_section)?;

        // Read and decode 18 byte Message Header Block
        let (input_header, message_header) = message_header(input_header)?;

        // fail if code is not in supported products list
        if !message_header.code.is_supported_product() {
            return Err(error_r::Error::ProductType(message_header.code).into())
        };

        // Read and decode 102 byte Product Description Block
        let (input_header, product_description) = product_description(input_header)?;

        if !input_header.is_empty() {
            warn!("Header leftovers: {:?}", input_header);
        }
        
        info!("{:?}", product_description);

        // Check product version number
        // if there is a supported version of this product type BUT (and) the product version is greater than the supported version
        if message_header.code.supported_version().is_some_and(|supported_version| product_description.version > supported_version) {
            return Err(Error::SupportedVersion(product_description.version, message_header.code.supported_version()))
        }

        // ---------------------------
        // File contents after header
        // ---------------------------

        let (input_data, symbology) = if product_description.offset_symbology > 0 {
            let (input_data, symbology) = symbology(remaining_file)?;
            (input_data, Some(symbology)) 
        } else {
            (remaining_file, None)
        };
        
        if product_description.offset_graphic > 0 {
            warn!("Should decode graphic alphanumeric block");
        }
        if product_description.offset_tabular > 0 {
            warn!("Should decode tabular alphanumeric block");
        }

        Ok((
            input_data,
            Radar {
                text_header,
                message_header,
                product_description,
                symbology,
                graphic: None,
                tabular: None,
            },
        ))
    }


    pub fn from_vec(mut file: Vec<u8>) -> Result<(Vec<u8>, Radar), Error> {
        let file_after_headers = file.split_off(150);

        // Uncompress symbology block if necessary
        let mut decomp_vec : Vec<u8> = Vec::new();
        if file_after_headers[0..1] == "BZ".as_bytes()[0..1] {
            let mut decoder = BzDecoder::new(file_after_headers.as_slice());
            let q = decoder.read_to_end(&mut decomp_vec);
            info!("Decompression {:?}", q);
        } else {
            // combine file after header back on to file???
            decomp_vec = file_after_headers;
        };
        
        info!("File is {:?} bytes.", file.len());
        match Radar::parse(&file, &decomp_vec) {
            Ok((leftover, radar)) => {
                Ok((leftover.to_vec(), radar))
            }
            Err(e) => {
                
                Err(e)
            },
        }
    }
}
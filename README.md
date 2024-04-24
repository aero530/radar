# radar processing test

## Spec Documents

[Interface Control Document Class 1 User](https://www.roc.noaa.gov/WSR88D/PublicDocs/ICDs/2620001AB.pdf)

[Interface Control Document Product Specification](https://www.roc.noaa.gov/WSR88D/PublicDocs/ICDs/2620003AB.pdf)

## Links

[RADAR PRODUCTS AVAILABLE FROM RPCCDS](https://www.weather.gov/media/tg/rpccds_radar_products.pdf)

[Satellite Product File Folder Structure](https://www.weather.gov/tg/satfiles)

[Radar Product File Folder Structure](https://www.weather.gov/tg/radfiles)

[NWS WSR-88D Level III Radar](https://www.roc.noaa.gov/WSR88D/Level_III/Level3Info.aspx)

Temperature - SL.us008001_ST.opnl_DF.gr2_DC.ndfd_AR.conus_VP.001-003_ds.temp


## Number formats

| | |
| --- | --- |
| Byte/Char   | One byte (8 bits) |
| INT*2       | 2 byte, signed integer data |
| INT*4       | 4 byte, signed integer data |
| UINT*4      | 4 byte, unsigned integer data |
| REAL*4      | 4 byte, floating point data adhering to IEEE-754-1985 standard |
| String      | NULL (0) terminated array of ASCII coded characters, each character occupying 1 byte  |
| Pointer     | Contains the address of a data item. Size is architecture dependent. |
| HALFWORD    | two bytes |


This is on page 3-27: 
```python
def _int16_to_float16(val):
    """Convert a 16 bit interger into a 16 bit float."""
    # NEXRAD Level III float16 format defined on page 3-33.
    # Differs from IEEE 768-2008 format so np.float16 cannot be used.
    sign = (val & 0b1000000000000000) / 0b1000000000000000
    exponent = (val & 0b0111110000000000) / 0b0000010000000000
    fraction = val & 0b0000001111111111
    if exponent == 0:
        return (-1) ** sign * 2 * (0 + (fraction / 2**10.0))
    else:
        return (-1) ** sign * 2 ** (exponent - 16) * (1 + fraction / 2**10.0)
```

## Format

**Figure 3-6. Graphic Product Message (Page 3-21)**
| Data format | |
| -- | -- |
| MESSAGE HEADER BLOCK | (see Figure 3-3) |
| PRODUCT DESCRIPTION BLOCK (1) | (see Figure 3-6 Sheet 2, 6, 7) |
| PRODUCT SYMBOLOGY BLOCK (1) | (see Figure 3-6 Sheet 3, 8) |
| GRAPHIC ALPHANUMERIC BLOCK (1) | (see Figure 3-6 Sheet 4, 9) |
| TABULAR ALPHANUMERIC BLOCK (1) | (see Figure 3-6 Sheet 5, 10) |

(1) All blocks need not be used. Any blocks that are used must remain in the order shown above.

### Graphic Product Message

The RPG transmits products to the Class 1 User/RPGOP by using the Graphic Product message shown in Figure 3-6. The message consists of several blocks. Not all products require all blocks; however, the blocks are always transmitted in the order shown in Figure 3-6. One Header block and one Product Description block always precede the product. Products consist of one Product Symbology block (Block ID = 1), and zero or one of each of the Graphic Alphanumeric (Block ID = 2), and Tabular Alphanumeric blocks (Block ID = 3). The number of the last two blocks in each message used is product dependent.

### Product Description Block

The Product Description block for product data transmission is shown in **Figure 3-6 (sheets 2, 6, and 7)**. Refer to Table V for the definitions of these fields and their corresponding products.

As shown in **Figure 3-6 (sheet 2)**, halfwords 55-60 contain offsets from the beginning of the message header (halfword 1) to the (-1) divider of each block indicated. If a product being transmitted does not require a block, or the data is not available, the offset to the block in question is set to zero. The first offset (halfword 55-56) is the offset to the Product Symbology block. The second offset (halfword 57-58) is the offset to the (-1) divider of the Graphic Alphanumeric block (Block ID = 2). The third offset is the offset to the Tabular Alphanumeric block (Block ID = 3).

### Product Symbology Block

The Product Symbology block is block ID number 1 and is shown in **Figure 3-6 (sheets 3 and 8)**. It is always numbered as 1. If it is available in a product, it will always follow the Product Description block. In general, this block contains display data packets that make up the geographic display of the product. These packets contain vectors, text and special character symbols, map data, radial data, raster data, precipitation data, vector arrow data, wind barb data, and special graphic symbols. The packet formats are defined in **Figures 3-7 through 3-15c**. The Symbology block may, depending upon the product, have multiple "layers" of packets.

### Graphic Alphanumeric Block

The Graphic Alphanumeric block is block ID number 2. It is the block in which display packets are
defined to cause the storm related data to be displayed at the top of the geographic screen to amplify
the corresponding graphic displayed symbology. The format of this block is shown graphically in
Figure 3-6 (sheets 4 and9). The only products for which this block is formatted are the following:

| Product Code | Product Name |
| -- | -- |
| 31 | User Selectable Precipitation |
| 37-38, 97-98 | Composite Reflectivity, Composite Reflectivity Edited for AP |
| 58 | Storm Tracking Information |
| 59 | Hail Index |
| 61 | Tornado Vortex Signature |
| 141 | Mesocyclone Detection |
| 143 | Tornado Vortex Signature Rapid Update |

The actual data within this block is a series of text packets that format the line data into 5 lines. The number of pages is data dependent. The text packet format used for the attributes is packet number 8 shown in Figure 3-8. Notice that I-start and J-start are defined as 1/4 km from the radar. The Graphic Attributes packets are not geographic, but are actual screen coordinates. Included in the text packet for each page of Attribute data is a series of vector packets to draw the grid lines. The vector packets used are shown in Figure 3-7. The product dependent data identified in Table VII is incorporated into the Graphic Alphanumeric Block.

### Tabular Alphanumeric Block

The Tabular Alphanumeric block for product data transmission is Block ID number 3. The format of this block is shown graphically in Figure 3-6 (sheets 5 and 10). It is always numbered 3 even though it may not be the third block in the product. The following products have a paired-alphanumeric product that is encoded as Block 3 (Figure 3-6, sheet 7). The paired-alphanumeric product has a second Header and Product Description block as shown in the figure. The products that have Block ID 3 are as follows:

| Product Code | Product Name | Block 3 Message Code |
| -- | -- | -- |
| 48 | VAD Wind Profile | 100 |
| 58 | Storm Tracking Information | 101 |
| 59 | Hail Index | 102 |
| 61 | Tornado Vortex Signature | 104 |
| 78 | Surface Rainfall Accumulation (1 hour) | 107 |
| 79 | Surface Rainfall Accumulation (3 hours) | 108 |
| 80 | Storm Total Rainfall Accumulation | 109 |
| 132 | Clutter Likelihood Reflectivity | 110 |
| 133 | Clutter Likelihood Doppler | 111 |
| 141 | Mesocyclone Detection | 141 |
| 143 | Tornado Vortex Signature Rapid Update | 143 |
| 172 | Digital Storm Total Accumulation | 172 |

The second header of the alphanumeric product is exactly the same as the header at the beginning of the message, except that the Message Code is as defined above. The Data portion of the alphanumeric product is ASCII text formatted into pages of 17 lines of 80-character data. Each page is separated by the (-1) divider. Alphanumeric products containing this block have it as the last block of the product message. The product dependent data identified in Table VIII is incorporated into the Tabular Alphanumeric Block.
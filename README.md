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

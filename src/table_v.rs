
/// "MSG_CODE"	HWORD#	CONTENT	UNITS	RANGE
// MSG CODE correlates to message_codes::MessageCode

152	51	Compression Method	N/A	0 or 1
152	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 500000
152	53	Uncompressed Product Data Size (LSW)		
202	51	Compression Method	N/A	0 or 1
202	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 500000
202	53	Uncompressed Product Data Size (LSW)		
94	30	Elevation Angle	Degree	-1.0 to +45.0
94	47	Max Reflectivity	dBZ	-32 to +95, (-33)
94	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15 (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
94	51	Compression Method	N/A	0 or 1
94	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 188000
94	53	Uncompressed Product Data Size (LSW)		
30	30	Elevation Angle	Degree	-1.0 to +45.0
30	47	Max Spectrum Width	Knots	0 to 19
30	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
99	30	Elevation Angle	Degree	-1.0 to +45.0
99	47	Max Neg. Velocity	Knots	-247 to 0
99	48	Max Pos. Velocity	Knots	0 to 245
99	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
99	51	Compression Method	N/A	0 or 1
99	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 372000
99	53	Uncompressed Product Data Size (LSW)		
132	30	Elevation Angle	Degree	-1.0 to +45.0
132	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
133	30	Elevation Angle	Degree	-1.0 to +45.0
133	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan  2 â€“ MRLE Scan"
113	27	RPG Cut Number	N/A	1 to 27
113	28	CMD Generated Flag	N/A	0 or 1
113	30	Elevation Angle	Degree	-1.0 to +45.0
113	47	Clutter Filter Map Time	Minutes	0 to 1439
113	48	Clutter Filter Map Date	Julian Date	1 to 32767
113	51	Compression Method	N/A	0 or 1
113	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 500000
113	53	Uncompressed Product Data Size (LSW)		
37 - 38	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
37 - 38	47	Max Reflectivity	dBZ	-32 to +95, (-33)
37 - 38	51	Cal. Constant (MSB)		
37 - 38	52	Cal Constant  (LSB)	dB (Real*4)	"-50.0 to +50.0, Note 14 -198.0 to +198.0, Note 15"
97-98	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
97 - 98	47	Max Reflectivity	dBZ	-32 to 95, (-33)
97 - 98	51	Cal Constant (MSB)		
97 - 98	52	Cal Constant  (LSB)	dB (Real*4)	"-50.0 to +50.0, Note 14 -198.0 to +198.0, Note 15"
51	47	Azimuth point one	Degree	0.0 to 359.9
51	48	Range point one	Nmi	0.0 to 124.0
51	49	Azimuth point two	Degree	0,0 to 359.9
51	50	Range point two	Nmi	0.0 to 124.0
50	47	Azimuth point one	Degree	0.0 to 359.9
50	48	Range point one	Nmi	0.0 to 124.0
50	49	Azimuth point two	Degree	0.0 TO 359.9
50	50	Range point two	Nmi	0.0 to 124.0
50	51	Cal. Constant (MSB)		
50	52	Cal. Constant (LSB)	dB (Real*4)	"-50.0 to +50.0, Note 14 -198.0 to +198.0, Note 15"
32	47	Max Reflectivity	dBZ	-32 to +95, (-33)
32	48	Date of Scan	Julian Date	1 to 32767
32	49	Avg. Time of Hybrid Scan	Minutes	0 to 1439
32	51	Compression Method	N/A	0 or 1
32	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 86000
32	53	Uncompressed Product Data Size (LSW)		
149	27	"Adaptation Data setting for Minimum Reflectivity Threshold"	dBZ	-25 to 35
149	30	Elevation Angle	Degree	-1.0 to + 45.0
149	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
149	51	Compression Method	N/A	0 or 1
149	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 300000
149	53	Uncompressed Product Data Size (LSW)		
193	30	Elevation Angle	Degree	-1.0 to +45.0
193	47	Max Reflectivity	dBZ	-31.5 to +95, (33)
193	48	"Number of artifact edited radials in elevation"	unitless	0 to 10000
193	49	AVSET Status	unitless	0, 1, 3
193	50	Chaff Detection Status	unitless	0, 1
193	51	Compression Method	N/A	0 or 1
193	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 1329150
193	53	Uncompressed Product Data Size (LSW)		
195	30	Elevation Angle	Degree	-1.0 to +45.0
195	47	Max Reflectivity	dBZ	-32 to +95, (-33)
195	48	"Number of artifact edited radials in elevation"	unitless	0 to 10000
195	49	AVSET Status	unitless	0, 1, 3
195	51	Compression Method	N/A	0 or 1
195	52	Uncompressed Product Data Size (MSW)	Bytes	770 - 167910
195	53	Uncompressed Product Data Size (LSW)		
196	27	Half Degree Scan Count Within Volume	N/A	0-1000
138	27	Beg. Date of Rainfall	Julian Date	1 to 32767
138	28	Beg.  Time of Rainfall	Minutes	0 to 1439
138	30	Mean-field Bias	N/A	0.0 to 99.99
138	47	Max Rainfall	Inches	0 to 51.00, Note 12
138	48	End Date of Rainfall	Julian Date	1 to 32767
138	49	End Time of Rainfall	Minutes	0 to 1439
138	50	Sample Size (No. G-R Pairs)	N/A	.00 to 99.99
138	51	Compression Method	N/A	0 or 1
138	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 300000
138	53	Uncompressed Product Data Size (LSW)		
41	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to + 45.0
41	47	Max Echo	1000 Feet	0 to 70
75	47	RPG ID Number	N/A	0 to 999
140	49	Detection count	N/A	0 - 1000
179	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
179	47	Maximum Hail top altitude in volume	kft	0 to 70
179	48	HSDA status	N/A	0 or 1
179	51	Compression Method	N/A	0 or 1
179	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 1329150
179	53	Uncompressed Product Data Size (LSW)		
59	--	--	--	--
135	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
135	47	Maximum echo top height in volume	kft	0 to 70
135	48	"Number of artifact edited radials in volume"	unitless	0 to 10000
135	49	Echo Tops reflectivity factor threshold	dBZ	-32 to 95
135	50	Number of spurious points removed	unitless	0 to 10000
135	51	Compression Method	N/A	0 or 1
135	52	Uncompressed Product Data Size (MSW)	Bytes	764 - 126870
135	53	Uncompressed Product Data Size (LSW)		
134	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
134	47	Max Digital VIL	unitless	0 to 254
134	48	"Number of artifact edited radials in volume"	unitless	0 to 10000
134	51	Compression Method	N/A	0 or 1
134	52	Uncompressed Product Data Size (MSW)	Bytes	770 - 167910
134	53	Uncompressed Product Data Size (LSW)		
81	47	Max Rainfall Accum.	dBA	-6.0 to 25.625
81	48	Mean-field Bias	N/A	0.01 to 99.99
81	49	Effective No. G-R Pairs (Sample Size)	N/A	0.00 to 99.99
81	50	Rainfall End Date	Julian Date	1 to 32767
81	51	Rainfall End Time	Minutes	0 to 1439
178	30	"AVSET termination elevation angle Otherwise = 0"	Degrees	-1.0 to +45.0
178	47	"Maximum icing top altitude in volume (graupel-based)"	kft	0 to 70
178	51	Compression Method	N/A	0 or 1
178	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 1329150
178	53	Uncompressed Product Data Size (LSW)		
93	30	Elevation Angle	Degree	-1.0 to +45.0
93	47	Max Neg. Velocity	Knots	-123 to 0
93	48	Max Pos. Velocity	Knots	0 to 122
93	50	Velocity Precision Code	N/A	1 or 2
65	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to 45.0
65	47	Max Reflectivity	dBZ	-32 to +95
65	48	Bottom of layer	1000 Feet	0
65	49	Top of layer	1000 Feet	6 to 58
65	51	Cal. Constant (MSB)		
65	52	Cal. Constant  (LSB)	dB (Real*4)	"-50.0 to +50.0, Note 14 -198.0 to +198.0, Note 15"
66	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
66	47	Max Reflectivity	dBZ	-32 to +95
66	48	Bottom of layer	1000 Feet	6 to 58
66	49	Top of layer	1000 Feet	12 to 64
66	51	Cal. Constant (MSB)		
66	52	Cal. Constant (LSB)	dB (Real*4)	"-50.0 to +50.0, Note 14 -198.0 to +198.0, Note 15"
67	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
67	47	Max Reflectivity	dBZ	-32 to +95
67	48	Bottom of layer	1000 Feet	0
67	49	Top of layer	1000 Feet	6 to 58
67	51	Cal. Constant (MSB)		
67	52	Cal. Constant           (LSB)	dB (Real*4)	"-50.0 to +50.0, Note 14 -198.0 to +198.0, Note 15"
90	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
90	47	Max Reflectivity	dBZ	-32 to +95
90	48	Bottom of layer	1000 Feet	12 to 64
90	49	Top of layer	1000 Feet	18 to 70
90	51	Cal. Constant  (MSB)		
90	52	Cal. Constant (LSB)	dB (Real*4)	"-50.0 to +50.0, Note 14 -198.0 to +198.0, Note 15"
141	27	"Adaptation Data setting for Minimum Reflectivity Threshold"	dBZ	-25 to 35
141	28	"Adaptation Data setting for Overlap Display Filter"	N/A	0 or 1
141	30	"Adaptation Data setting for Minimum Display Filter Strength Rank"	N/A	1 to 5
196	49	Detection Count	NA	0-1000
144	27	Length of Missing Periods	Minutes	0 to 32767
144	30	Use RCA Flag	N/A	0 or 1
144	47	Maximum Value	Inches	0.001 to 32.767
144	48	Starting Date	Julian Date	1 to 32767
144	49	Starting Time	Minutes	0 to 1439
144	50	Ending Date	Julian Date	1 to 32767
144	51	Ending Time	Minutes	0 to 1439
144	52	Azimuth of Max.	Degrees	0 to 359
144	53	Range to Max.	Nmi	0 to 124
145	27	Length of Missing Periods	Minutes	0 to 32767
145	30	Use RCA Flag	N/A	0 or 1
145	47	Maximum Value	Inches	0.01 to 327.67
145	48	Starting Date	Julian Date	1 to 32767
145	49	Starting Time	Minutes	0 to 1439
145	50	Ending Date	Julian Date	1 to 32767
145	51	Ending Time	Minutes	0 to 1439
145	52	Azimuth of Max	Degrees	0 to 359
145	53	Range to Max.	Nmi	0 to 124
56	30	Elevation Angle	Degree	-1.0 to +45.0
56	47	Max Neg. Velocity	Knots	-247 to 0
56	48	Max Pos. Velocity	Knots	0 to +245
56	49	Motion Source Flag	N/A	#NAME?
56	51	Avg Speed of Storms	Knots	0.0 to 99.9
56	52	Avg Dir. of Storms	Degree	0.0 to 359.9
62	--	--	--	
80	47	Max Rainfall	Inches	0.0 to 327.6
80	48	Beg. Date Rainfall	Julian Date	1 to 32767
80	49	Beg. Time Rainfall	Minutes	0 to 1439
80	50	End Date Rainfall	Julian date	1 to 32767
80	51	End Time Rainfall	Minutes	0 to 1439
80	52	Mean-field Bias	N/A	0.01 to 99.99
80	53	Effective No. G-R Pairs (Sample Size)	N/A	0.00 to 99.99
147	27	Length of Missing Periods	Minutes	0 to ??
147	30	Use RCA Flag	N/A	0 or 1
147	47	Maximum Value	Inches	0.0 to 3276.7
147	48	Starting Date	Julian Date	1 to 32767
147	49	Starting Time	Minutes	0 to 1439
147	50	Ending Date	Julian Date	1 to 32767
147	51	Ending Time	Minutes	0 to 1439
147	52	Azimuth of Max.	Degrees	0 to 359
147	53	Range to Max.	Nmi	0 to 124
146	27	Length of Missing Periods	Minutes	0 to 32767
146	30	Use RCA Flag	N/A	0 or 1
146	47	Maximum Value	Inches	0.00 to 327.67
146	48	Starting Date	Julian Date	1 to 32767
146	49	Starting Time	Minutes	0 to 1439
146	50	Ending Date	Julian Date	1 to 32767
146	51	Ending Time	Minutes	0 to 1439
146	52	Azimuth of Max.	Degrees	0 to 359
146	53	Range to Max.	Nmi	0 to 124
58	47	Total Number of Storms	N/A	0 to 100
153	30	Elevation Angle	Degree	-1.0 to +45.0
153	47	Max Reflectivity	dBZ	-32 to +95, (-33)
153	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
153	51	Compression Method	N/A	0 or 1
153	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 1329150
153	53	Uncompressed Product Data Size (LSW)		
154	30	Elevation Angle	Degree	-1.0 to +45.0
154	47	Max Neg. Velocity	Knots	-247 to 0
154	48	Max Pos. Velocity	Knots	0 to 245
154	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
154	51	Compression Method	N/A	0 or 1
154	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 868350
154	53	Uncompressed Product Data Size (LSW)		
155	30	Elevation Angle	Degree	-1.0 to +45.0
155	47	Max Spectrum Width	Knots	0 to 19
155	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
155	51	Compression Method	N/A	0 or 1
155	52	Uncompressed Product Data Size (MSW)	Bytes	120 to 868350
155	53	Uncompressed Product Data Size (LSW)		
78 & 79	47	Max Rainfall	Inches	0.0 to 189.0
78 & 79	48	Mean-field Bias	N/A	0.01 to 99.99
78 & 79	49	Effective No. G-R Pairs (Sample Size)	N/A	0.00 to 99.99
78 & 79	50	Rainfall End Date	Julian Date	1 to 32767
78 & 79	51	Rainfall End Time	Minutes	0 to 1439
61	47	Total Number of TVS	N/A	-25 to 25
61	48	Total Number of ETVS	N/A	-25 to 25
143	30	Elevation angle	degree	-1.0 to +45.0
143	47	Total Number of TVS	N/A	-25 to 25
143	48	Total Number of ETVS	N/A	-25 to 25
143	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
137	27	Requested Bottom Altitude of Layer	K Feet	0 to 69
137	28	Requested Top Altitude of Layer	K Feet	1 to 70
137	47	Max Reflectivity	dBZ	-32 to 95
137	48	"Actual bottom Altitude of Layer (adjusted to correct request errors)."	K Feet	0 to 69
137	49	"Actual top Altitude of Layer (adjusted to correct request errors)."	K Feet	1 to 70
31	27	End Hour	Hours	0 to 23
31	28	Time Span	Hours	1 to 24
31	30	Null Product Flag	N/A	0 to 1
31	47	Max Rainfall	Inches	0.0 to 327.6
31	48	Beg. Date Rainfall	Julian Date	1 to 32767
31	49	Beg. Time Rainfall	Minutes	0 to 1439
31	50	End Date Rainfall	Julian Date	1 to 32767
31	51	End Time Rainfall	Minutes	0 to 1439
31	52	Average Mean-field Bias	N/A	0.01 to 99.99
31	53	"Average Effective No. G-R Pairs (SampleSize)"	N/A	0.00 to 99.99
151	27	End Hour	Hours	0 to 23
151	28	Time Span	Hours	1 to 30
151	30	"Use High Scale Flag/ Use RCA Flag"	N/A	0, 1, 256, or 257
151	47	Maximum Value	Inches	"0.00 to 327.67 or 0.0 to 3276.7"
151	48	Starting Date	Julian Date	1 to 32767
151	49	Starting Hour	Minutes	0 to 1439
151	50	Ending Date	Julian Date	1 to 32767
151	51	Ending Hour	Minutes	0 to 1439
151	52	Azimuth of Max.	Degrees	0 to 359
151	53	Range to Max.	Nmi	0 to 124
150	27	End Hour	Hours	0 to 23
150	28	Time Span	Hours	1 to 30
150	30	Use High Scale Flag/ Use RCA Flag	N/A	0, 1, 256, or 257
150	47	Maximum Value	Inches	"0.000 to 32.767 or 0.00 to 327.67"
150	48	Starting Date	Julian Date	1 to 32767
150	49	Starting Hour	Minutes	0 to 1439
150	50	Ending Date	Julian Date	1 to 32767
150	51	Ending Hour	Minutes	0 to 1439
150	52	Azimuth of Max.	Degrees	0 to 359
150	53	Range to Max.	Nmi	0 to 124
48	47	Max Speed  (Horiz)	Knots	0 to 350
48	48	Direct of Max Speed	Degree	0 to 359
48	49	Alt of Max Speed	Feet/10	00.00 to 70.00
84	47	Wind Speed (Horiz)	Knots	0 to 350
84	48	Wind Direct(Horiz)	Degree	0 to 359
84	30	Wind Alt   (Horiz)	1000 Feet	0 to 70
84	49	Elevation Angle	Degree	-1.0 to +45.0
84	50	Slant Range	Nmi	0.0 to 124.0
84	51	RMS Error	Knots	0 to 29
57	30	"AVSET termination elevation angle Otherwise = 0"	Degree	-1.0 to +45.0
57	47	Max VIL	"Kg/Sq. meter"	0 to 200
159	30	Elevation Angle	Degree	-1.0 to +45.0
159	47	Minimum Differential Reflectivity	dB	-7.9 to +7.9
159	48	Maximum Differential Reflectivity	dB	-7.9 to +7.9
159	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
159	51	Compression method	N/A	0 or 1
159	52	Size of uncompressed product (MSW)	Bytes	120 to 434406
159	53	Size of uncompressed product (LSW)	Bytes	
161	30	Elevation Angle	Degree	-1.0 to +45.0
161	47	Minimum Correlation Coefficient	N/A	0.2 to 1.05
161	48	Maximum Correlation Coefficient	N/A	0.2 to 1.05
161	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
161	51	Compression Method	N/A	0 or 1
161	52	Size of uncompressed product (MSW)	Bytes	120 to 500000
161	53	Size of uncompressed product (LSW)	Bytes	
163	30	Elevation Angle	Degree	-1.0 to +45.0
163	47	Minimum Specific Differential Phase	Deg/km	-2.05 to +10.00
163	48	Maximum Specific Differential Phase	Deg/km	-2.05 to +10.00
163	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
163	51	Compression Method	N/A	0 or 1
163	52	Size of uncompressed product (MSW)	Bytes	120 to 500000
163	53	Size of uncompressed product (LSW)	Bytes	
165	30	Elevation Angle	Degree	-1.0 to +45.0
165	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
165	51	Compression Method	N/A	0 or 1
165	52	Size of uncompressed product (MSW)	Bytes	120 to 500000
165	53	Size of uncompressed product (LSW)	Bytes	
166	30	Elevation Angle	Degree	-1.0 to +45.0
166	47	Minimum Melting Layer Height	kft	1 to 70
166	48	Maximum Melting Layer Height	kft	1 to 70
166	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
167	30	Elevation Angle	Degrees	-1.0 to + 45.0
167	47	Min Correlation Coefficient	N/A	0.2 to 1.05
167	48	Max Correlation Coefficient	N/A	0.2 to 1.05
167	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
167	51	Compression Method	N/A	0 or 1
167	52	Size of uncompressed product (MSW)	Bytes	120 to 500000
167	53	Size of uncompressed product (LSW)	Bytes	
168	30	Elevation Angle	Degrees	-1.0 to + 45.0
168	47	Min Differential Phase	Degrees	0 to 360
168	48	Max Differential Phase	Degrees	0 to 360
168	50	Delta Time / Supplemental Scan	"Seconds / N/A"	"Bits 5-15: (0-800) Bits 0-4: 0 â€“ Non Supplemental Scan 1 â€“ SAILS Scan 2 â€“ MRLE Scan"
168	51	Compression Method	N/A	0 or 1
168	52	Size of uncompressed product (MSW)	Bytes	"120 to 500000120 to 500000"
168	53	Size of uncompressed product (LSW)	Bytes	
169	30	Null Product Flag	N/A	0 to 5
169	47	Max Accum	Inches	0.0 to 100.0
169	48	Ending Date of Accumulation	Julian Date	1 to 32767
169	49	Ending Time of Accumulation	Minutes	0 to 1439
169	50	Mean-field Bias	N/A	0.01 to 99.99
169	51	"Sample Size (Effective No. Gage/Radar
Pairs)"	N/A	0.00 to 99.99
170	27	Threshold Min. Time in Hourly Period	Minutes	0 to 60
170	28	Total Time in Hourly Period	Minutes	0 to 60
170	30	Null Product Flag	N/A	0 to 5
170	47	Max Accum	Inches	0.0 to 100.0
170	48	Ending Date of Accumulation	Julian Date	1 to 32767
170	49	Ending Time of Accumulation	Minutes	0 to 1439
170	50	Mean-field Bias	N/A	0.01 to 99.99
170	51	Compression Method	N/A	0 or 1
170	52	Size of uncompressed product (MSW)	Bytes	284 to 335096
170	53	Size of uncompressed product (LSW)	Bytes	
172	27	Start Date of Accumulation	Julian Date	1 to 32767
172	28	Start Time of Accumulation	Minutes	0 to 1439
172	30	Null Product Flag	N/A	0 to 5
172	47	Max Accum	Inches	0 to 100.00
172	48	Ending Date of Accumulation	Julian Date	1 to 32767
172	49	Ending Time of Accumulation	Minutes	0 to 1439
172	50	Mean-field Bias	N/A	0.0 to 99.99
172	51	Compression Method	N/A	0 or 1
172	52	Size of uncompressed product (MSW)	Bytes	916 to 355096
172	53	Size of uncompressed product (LSW)	Bytes	
173	27	End Time	Minutes	0 to 1439
173	28	Time Span Minutes	Minutes	15 to 1440
173	30	"Missing Period Flag (high byte) & Null Product Flag (low byte)"	N/A	"0 or 1 in the high byte; 0, 2 or 3 in the low byte"
173	47	Max Accum	Inches	0.0 to 327.6
173	48	End Date	Julian Date	1 to 32767
173	49	Start Time	Minutes	0 to 1439
173	50	Mean-field Bias	N/A	0.01 to 99.99
173	51	Compression Method	N/A	0 or 1
173	52	Size of uncompressed product (MSW)	Bytes	296 to 335096
173	53	Size of uncompressed product (LSW)	Bytes	
174	47	Max Accum Difference	Inches	-100.0 to 100.0
174	48	Ending Date of Accumulation	Julian Date	1 to 32767
174	49	Ending Time of Accumulation	Minutes	0 to 1439
174	50	Min Accum Difference	Inches	-100.0 to 100.0
174	51	Compression Method	N/A	0 or 1
174	52	Size of uncompressed product (MSW)	Bytes	2836 to 335096
174	53	Size of uncompressed product (LSW)	Byte	
175	27	Start Date of Accumulation	Julian Date	1 to 32767
175	28	Start Time of Accumulation	Minutes	0 to 1439
175	30	Null Product Flag	N/A	0 to 5
175	47	Max Accum Difference	Inches	-100.0 to 100.0
175	48	Ending Date of Accumulation	Julian date	1 to 32767
175	49	Ending Time of Accumulation	Minutes	0 to 1439
175	50	Min Accum Difference	Inches	-100.0 to 100.0
175	51	Compression Method	N/A	0 or 1
175	52	Size of uncompressed product (MSW)	Bytes	2836 to 335096
175	53	Size of uncompressed product (LSW)	Bytes	
176	27	Hybrid Rate Scan Date	Julian date	1 to 32767
176	28	Hybrid Rate Scan Time	Minutes	0 to 1439
176	30	"Precipitation Detected Flag (high byte) & Gage Bias to be Applied Flag (low byte)"	N/A	0 or 1
176	47	"Maximum Instantaneous PrecipitationRate"	in/hr	0 to 65535
176	48	Hybrid Rate Percent Bins Filled	Percent	0.01 - 100.00
176	49	Highest Elev. Used	Degrees	0.5 - 19.5
176	50	Mean-field Bias	N/A	0.00 to 99.99
176	51	Compression Method	N/A	0 or 1
176	52	Size of uncompressed product (MSW)	Bytes	1627 to 662496
176	53	Size of uncompressed product (LSW)	Bytes	
177	47	Mode Filter Size	N/A	1 to 15
177	48	Hybrid Rate Percent Bins Filled	Percent	0.01 - 100.00
177	49	Highest Elev. Used	Degrees	0.5 - 19.5
177	51	Compression Method	N/A	0 or 1
177	52	Size of uncompressed product (MSW)	Bytes	120 to 500000
177	53	Size of uncompressed product (LSW)	Bytes	
197	47	Mode Filter Size	N/A	1 to 15
197	48	Rain Rate Percent Bins Filled	Percent	0.01     100.00
197	49	Highest Elev. Used	Degrees	0.5 - 19.5
197	50	Multiplier for Dry Snow above the ML	N/A	1.0 to 2.8
197	51	Compression Method	N/A	0or 1
197	52	Size of uncompressed product (MSW)	Bytes	120 to 500000
197	53	Size of uncompressed product (LSW)	Bytes	

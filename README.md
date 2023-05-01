# excelpivotdata
Rust library and Python Wrapper to read Excel Pivot Data.

Why did you do this ?

At work a collegue mentioned they had a need to get the data from a pivot table. The only available python library which read pivot table data in python
took over 5 minutes.

This library reads the pivot data in an XLSX file using an event based approach. The data is made available via polars to python.
The performance is good with a test spreadsheet containing 2.1 million rows being parsed and a polars dataframe returned to python in 7 seconds.


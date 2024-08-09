# awr2544-pac
Peripheral Access Crate (PAC) for the Texas Instruments (TI) AWR2544 mmwave radar.  Generated using tixml2svd and svd2rust.  


# Commands to regenerate
```shell
tixml2svd.exe -z -i "C:\ti\ccs1271\ccs\ccs_base\common\targetdb\devices\awr2544.xml" >awr2544.svd
svd2rust -i xml\iwrl6432_updated.svd --target=cortex-r
form -i lib.rs -o src/
cargo fmt
```
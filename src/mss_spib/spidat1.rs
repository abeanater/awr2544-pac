#[doc = "Register `SPIDAT1` reader"]
pub type R = crate::R<Spidat1Spec>;
#[doc = "Register `SPIDAT1` writer"]
pub type W = crate::W<Spidat1Spec>;
#[doc = "Field `TXDATA` reader - 15:0\\]
SPI / MibSPI Transmit Data. When written, these bits will be copied to the Shift Register if it is empty. If the Shift Register is not empty, the TXBUF will hold the written values. SPIEN must be set to 1 before this register can be written to. Writing a 0 to the SPIEN register forces the lower 16 bits of the SPIDAT1 register to 0x00. Write to this register ONLY when using the automatic Slave Chip Select feature. See section 3, Operation Modes, on page 14. A write to this register will drive the contents of CSNR\\[7:0\\]
into the respective pins in SPISCS\\[7:0\\]
if those are configured as functional pins. When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned. Irrespective of the character length, the Transmit data should be right justified before writing to SPIDAT1 register."]
pub type TxdataR = crate::FieldReader<u16>;
#[doc = "Field `TXDATA` writer - 15:0\\]
SPI / MibSPI Transmit Data. When written, these bits will be copied to the Shift Register if it is empty. If the Shift Register is not empty, the TXBUF will hold the written values. SPIEN must be set to 1 before this register can be written to. Writing a 0 to the SPIEN register forces the lower 16 bits of the SPIDAT1 register to 0x00. Write to this register ONLY when using the automatic Slave Chip Select feature. See section 3, Operation Modes, on page 14. A write to this register will drive the contents of CSNR\\[7:0\\]
into the respective pins in SPISCS\\[7:0\\]
if those are configured as functional pins. When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned. Irrespective of the character length, the Transmit data should be right justified before writing to SPIDAT1 register."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CSNR` reader - 23:16\\]
Chip select number. CSNR defines the chip select that shall be activated during the data transfer. The value of CSNR\\[7:0\\]
will be driven on SPISCS\\[7:0\\]
lines during the transfer. Note: Effect of NUM_CS_PINS generic on CSNR bits. Actual number of bits implemented in CSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Preselecting a Format Register. Writing to just the Control Field (using byte writes) does not initiate any SPI transfer in Master mode. This feature can be used to setup SPICLK Phase or Polarity before actually starting the transfer by just updating the DFSEL fields in the control field to select the required Phase/Polarity combination."]
pub type CsnrR = crate::FieldReader;
#[doc = "Field `CSNR` writer - 23:16\\]
Chip select number. CSNR defines the chip select that shall be activated during the data transfer. The value of CSNR\\[7:0\\]
will be driven on SPISCS\\[7:0\\]
lines during the transfer. Note: Effect of NUM_CS_PINS generic on CSNR bits. Actual number of bits implemented in CSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Preselecting a Format Register. Writing to just the Control Field (using byte writes) does not initiate any SPI transfer in Master mode. This feature can be used to setup SPICLK Phase or Polarity before actually starting the transfer by just updating the DFSEL fields in the control field to select the required Phase/Polarity combination."]
pub type CsnrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DFSEL` reader - 25:24\\]
DFSEL1 DFSEL0 Description 0 0 Data word format 0 is selected 0 1 Data word format 1 is selected 1 0 Data word format 2 is selected 1 1 Data word format 3 is selected"]
pub type DfselR = crate::FieldReader;
#[doc = "Field `DFSEL` writer - 25:24\\]
DFSEL1 DFSEL0 Description 0 0 Data word format 0 is selected 0 1 Data word format 1 is selected 1 0 Data word format 2 is selected 1 1 Data word format 3 is selected"]
pub type DfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDEL` reader - 26:26\\]
Enable the delay counter at the end of the current transaction. WDELAY bit is supported in Master mode only. In Slave mode, this bit will be ignored. 1 = After a transaction, WDELAY of the corresponding data format will be loaded into the delay counter. No transaction will be performed until the WDELAY counter overflows. The SPISCS pins will be de-activated for atleast (WDELAY + 2) * VCLK_Period duration. 0 = No delay will be inserted. However, SPISCS pins will still be de-activated for atleast for 2VCLK cycles if CSHOLD = ΓÇÿ0ΓÇÖ. In SPI or Compatibility mode of MibSPI, the duration for which the SPISCS pin remaining de-activated will also depend upon time taken to supply a new data after completing the shifting operation. If the internal buffer - TXBUF is already full, then the SPISCS will be deasserted for atleast 2 VCLK cycles(if WDEL = ΓÇÿ0ΓÇÖ)."]
pub type WdelR = crate::BitReader;
#[doc = "Field `WDEL` writer - 26:26\\]
Enable the delay counter at the end of the current transaction. WDELAY bit is supported in Master mode only. In Slave mode, this bit will be ignored. 1 = After a transaction, WDELAY of the corresponding data format will be loaded into the delay counter. No transaction will be performed until the WDELAY counter overflows. The SPISCS pins will be de-activated for atleast (WDELAY + 2) * VCLK_Period duration. 0 = No delay will be inserted. However, SPISCS pins will still be de-activated for atleast for 2VCLK cycles if CSHOLD = ΓÇÿ0ΓÇÖ. In SPI or Compatibility mode of MibSPI, the duration for which the SPISCS pin remaining de-activated will also depend upon time taken to supply a new data after completing the shifting operation. If the internal buffer - TXBUF is already full, then the SPISCS will be deasserted for atleast 2 VCLK cycles(if WDEL = ΓÇÿ0ΓÇÖ)."]
pub type WdelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 27:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - 27:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSHOLD` reader - 28:28\\]
Chip select hold mode. In SPI or compatibility mode MibSPI, the CSHOLD bit is supported in master mode only. In slave mode, this bit is ignored. CSHOLD defines the behavior of the chip select line at the end of a data transfer. 1 =The chip select signal is held active at the end of a transfer until a control field with new data and control information is loaded into SPIDAT1. If the new chip select information equals the previous one, the active chip select signal is extended until the end of transfer with CSHOLD cleared or until the chip select information changes. 0 =The chip select signal is deactivated at the end of a transfer after the T2CDELAY time has passed. If two consecutive transfers are dedicated to the same chip select this chip select signal will be deactivated for atleast 2VCLK cycles before it is activated again."]
pub type CsholdR = crate::BitReader;
#[doc = "Field `CSHOLD` writer - 28:28\\]
Chip select hold mode. In SPI or compatibility mode MibSPI, the CSHOLD bit is supported in master mode only. In slave mode, this bit is ignored. CSHOLD defines the behavior of the chip select line at the end of a data transfer. 1 =The chip select signal is held active at the end of a transfer until a control field with new data and control information is loaded into SPIDAT1. If the new chip select information equals the previous one, the active chip select signal is extended until the end of transfer with CSHOLD cleared or until the chip select information changes. 0 =The chip select signal is deactivated at the end of a transfer after the T2CDELAY time has passed. If two consecutive transfers are dedicated to the same chip select this chip select signal will be deactivated for atleast 2VCLK cycles before it is activated again."]
pub type CsholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 31:29\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 31:29\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
SPI / MibSPI Transmit Data. When written, these bits will be copied to the Shift Register if it is empty. If the Shift Register is not empty, the TXBUF will hold the written values. SPIEN must be set to 1 before this register can be written to. Writing a 0 to the SPIEN register forces the lower 16 bits of the SPIDAT1 register to 0x00. Write to this register ONLY when using the automatic Slave Chip Select feature. See section 3, Operation Modes, on page 14. A write to this register will drive the contents of CSNR\\[7:0\\]
into the respective pins in SPISCS\\[7:0\\]
if those are configured as functional pins. When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned. Irrespective of the character length, the Transmit data should be right justified before writing to SPIDAT1 register."]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Chip select number. CSNR defines the chip select that shall be activated during the data transfer. The value of CSNR\\[7:0\\]
will be driven on SPISCS\\[7:0\\]
lines during the transfer. Note: Effect of NUM_CS_PINS generic on CSNR bits. Actual number of bits implemented in CSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Preselecting a Format Register. Writing to just the Control Field (using byte writes) does not initiate any SPI transfer in Master mode. This feature can be used to setup SPICLK Phase or Polarity before actually starting the transfer by just updating the DFSEL fields in the control field to select the required Phase/Polarity combination."]
    #[inline(always)]
    pub fn csnr(&self) -> CsnrR {
        CsnrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
DFSEL1 DFSEL0 Description 0 0 Data word format 0 is selected 0 1 Data word format 1 is selected 1 0 Data word format 2 is selected 1 1 Data word format 3 is selected"]
    #[inline(always)]
    pub fn dfsel(&self) -> DfselR {
        DfselR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - 26:26\\]
Enable the delay counter at the end of the current transaction. WDELAY bit is supported in Master mode only. In Slave mode, this bit will be ignored. 1 = After a transaction, WDELAY of the corresponding data format will be loaded into the delay counter. No transaction will be performed until the WDELAY counter overflows. The SPISCS pins will be de-activated for atleast (WDELAY + 2) * VCLK_Period duration. 0 = No delay will be inserted. However, SPISCS pins will still be de-activated for atleast for 2VCLK cycles if CSHOLD = ΓÇÿ0ΓÇÖ. In SPI or Compatibility mode of MibSPI, the duration for which the SPISCS pin remaining de-activated will also depend upon time taken to supply a new data after completing the shifting operation. If the internal buffer - TXBUF is already full, then the SPISCS will be deasserted for atleast 2 VCLK cycles(if WDEL = ΓÇÿ0ΓÇÖ)."]
    #[inline(always)]
    pub fn wdel(&self) -> WdelR {
        WdelR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Chip select hold mode. In SPI or compatibility mode MibSPI, the CSHOLD bit is supported in master mode only. In slave mode, this bit is ignored. CSHOLD defines the behavior of the chip select line at the end of a data transfer. 1 =The chip select signal is held active at the end of a transfer until a control field with new data and control information is loaded into SPIDAT1. If the new chip select information equals the previous one, the active chip select signal is extended until the end of transfer with CSHOLD cleared or until the chip select information changes. 0 =The chip select signal is deactivated at the end of a transfer after the T2CDELAY time has passed. If two consecutive transfers are dedicated to the same chip select this chip select signal will be deactivated for atleast 2VCLK cycles before it is activated again."]
    #[inline(always)]
    pub fn cshold(&self) -> CsholdR {
        CsholdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
SPI / MibSPI Transmit Data. When written, these bits will be copied to the Shift Register if it is empty. If the Shift Register is not empty, the TXBUF will hold the written values. SPIEN must be set to 1 before this register can be written to. Writing a 0 to the SPIEN register forces the lower 16 bits of the SPIDAT1 register to 0x00. Write to this register ONLY when using the automatic Slave Chip Select feature. See section 3, Operation Modes, on page 14. A write to this register will drive the contents of CSNR\\[7:0\\]
into the respective pins in SPISCS\\[7:0\\]
if those are configured as functional pins. When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned. Irrespective of the character length, the Transmit data should be right justified before writing to SPIDAT1 register."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<Spidat1Spec> {
        TxdataW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Chip select number. CSNR defines the chip select that shall be activated during the data transfer. The value of CSNR\\[7:0\\]
will be driven on SPISCS\\[7:0\\]
lines during the transfer. Note: Effect of NUM_CS_PINS generic on CSNR bits. Actual number of bits implemented in CSNR\\[7:0\\]
will depend upon the NUM_CS_PINS generic set during synthesis. Unimplemented bits will be read-only and will read ΓÇÿ0ΓÇÖ always. Note: Preselecting a Format Register. Writing to just the Control Field (using byte writes) does not initiate any SPI transfer in Master mode. This feature can be used to setup SPICLK Phase or Polarity before actually starting the transfer by just updating the DFSEL fields in the control field to select the required Phase/Polarity combination."]
    #[inline(always)]
    #[must_use]
    pub fn csnr(&mut self) -> CsnrW<Spidat1Spec> {
        CsnrW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
DFSEL1 DFSEL0 Description 0 0 Data word format 0 is selected 0 1 Data word format 1 is selected 1 0 Data word format 2 is selected 1 1 Data word format 3 is selected"]
    #[inline(always)]
    #[must_use]
    pub fn dfsel(&mut self) -> DfselW<Spidat1Spec> {
        DfselW::new(self, 24)
    }
    #[doc = "Bit 26 - 26:26\\]
Enable the delay counter at the end of the current transaction. WDELAY bit is supported in Master mode only. In Slave mode, this bit will be ignored. 1 = After a transaction, WDELAY of the corresponding data format will be loaded into the delay counter. No transaction will be performed until the WDELAY counter overflows. The SPISCS pins will be de-activated for atleast (WDELAY + 2) * VCLK_Period duration. 0 = No delay will be inserted. However, SPISCS pins will still be de-activated for atleast for 2VCLK cycles if CSHOLD = ΓÇÿ0ΓÇÖ. In SPI or Compatibility mode of MibSPI, the duration for which the SPISCS pin remaining de-activated will also depend upon time taken to supply a new data after completing the shifting operation. If the internal buffer - TXBUF is already full, then the SPISCS will be deasserted for atleast 2 VCLK cycles(if WDEL = ΓÇÿ0ΓÇÖ)."]
    #[inline(always)]
    #[must_use]
    pub fn wdel(&mut self) -> WdelW<Spidat1Spec> {
        WdelW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Spidat1Spec> {
        Nu1W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Chip select hold mode. In SPI or compatibility mode MibSPI, the CSHOLD bit is supported in master mode only. In slave mode, this bit is ignored. CSHOLD defines the behavior of the chip select line at the end of a data transfer. 1 =The chip select signal is held active at the end of a transfer until a control field with new data and control information is loaded into SPIDAT1. If the new chip select information equals the previous one, the active chip select signal is extended until the end of transfer with CSHOLD cleared or until the chip select information changes. 0 =The chip select signal is deactivated at the end of a transfer after the T2CDELAY time has passed. If two consecutive transfers are dedicated to the same chip select this chip select signal will be deactivated for atleast 2VCLK cycles before it is activated again."]
    #[inline(always)]
    #[must_use]
    pub fn cshold(&mut self) -> CsholdW<Spidat1Spec> {
        CsholdW::new(self, 28)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Spidat1Spec> {
        Nu2W::new(self, 29)
    }
}
#[doc = "SPI / MibSPI Transmit Data Register 1 When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned.\n\nYou can [`read`](crate::Reg::read) this register and get [`spidat1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spidat1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spidat1Spec;
impl crate::RegisterSpec for Spidat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spidat1::R`](R) reader structure"]
impl crate::Readable for Spidat1Spec {}
#[doc = "`write(|w| ..)` method takes [`spidat1::W`](W) writer structure"]
impl crate::Writable for Spidat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIDAT1 to value 0"]
impl crate::Resettable for Spidat1Spec {
    const RESET_VALUE: u32 = 0;
}

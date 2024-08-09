#[doc = "Register `SPIDAT0` reader"]
pub type R = crate::R<Spidat0Spec>;
#[doc = "Register `SPIDAT0` writer"]
pub type W = crate::W<Spidat0Spec>;
#[doc = "Field `TXDATA` reader - 15:0\\]
SPI / MibSPI Transmit Data. When written, these bits will be copied to the Shift Register if it is empty. If the Shift Register is not empty, the TXBUF will hold the written values. SPIEN (SPICGR1.24) must be set to 1 before this register can be written to. Writing a 0 to the SPIEN register forces the lower 16 bits of the SPIDAT0 register to 0x00. When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned. As the data is shifted out from either the MSB or the LSB of Transmit Shift Register depending upon SHIFTDIR bit (SPIFMTx.20). Simultaneously, the received bit will be shifted into the Receive Shift Register either through LSB or MSB depending upon SHIFTDIR bit. This allows the concurrent transmission and reception of data. Note: Irrespective of the character length, the Transmit data should be right justified before writing to SPIDAT0 register. The default Data Format Control register for SPIDAT0 is SPIFMT0. However it is possible to reprogram the DFSEL\\[1:0\\]
fields of SPIDAT1 prior to using SPIDAT0, to select a different SPIFMTx register."]
pub type TxdataR = crate::FieldReader<u16>;
#[doc = "Field `TXDATA` writer - 15:0\\]
SPI / MibSPI Transmit Data. When written, these bits will be copied to the Shift Register if it is empty. If the Shift Register is not empty, the TXBUF will hold the written values. SPIEN (SPICGR1.24) must be set to 1 before this register can be written to. Writing a 0 to the SPIEN register forces the lower 16 bits of the SPIDAT0 register to 0x00. When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned. As the data is shifted out from either the MSB or the LSB of Transmit Shift Register depending upon SHIFTDIR bit (SPIFMTx.20). Simultaneously, the received bit will be shifted into the Receive Shift Register either through LSB or MSB depending upon SHIFTDIR bit. This allows the concurrent transmission and reception of data. Note: Irrespective of the character length, the Transmit data should be right justified before writing to SPIDAT0 register. The default Data Format Control register for SPIDAT0 is SPIFMT0. However it is possible to reprogram the DFSEL\\[1:0\\]
fields of SPIDAT1 prior to using SPIDAT0, to select a different SPIFMTx register."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - 31:16\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 31:16\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
SPI / MibSPI Transmit Data. When written, these bits will be copied to the Shift Register if it is empty. If the Shift Register is not empty, the TXBUF will hold the written values. SPIEN (SPICGR1.24) must be set to 1 before this register can be written to. Writing a 0 to the SPIEN register forces the lower 16 bits of the SPIDAT0 register to 0x00. When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned. As the data is shifted out from either the MSB or the LSB of Transmit Shift Register depending upon SHIFTDIR bit (SPIFMTx.20). Simultaneously, the received bit will be shifted into the Receive Shift Register either through LSB or MSB depending upon SHIFTDIR bit. This allows the concurrent transmission and reception of data. Note: Irrespective of the character length, the Transmit data should be right justified before writing to SPIDAT0 register. The default Data Format Control register for SPIDAT0 is SPIFMT0. However it is possible to reprogram the DFSEL\\[1:0\\]
fields of SPIDAT1 prior to using SPIDAT0, to select a different SPIFMTx register."]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
SPI / MibSPI Transmit Data. When written, these bits will be copied to the Shift Register if it is empty. If the Shift Register is not empty, the TXBUF will hold the written values. SPIEN (SPICGR1.24) must be set to 1 before this register can be written to. Writing a 0 to the SPIEN register forces the lower 16 bits of the SPIDAT0 register to 0x00. When this register is read, contents of internal buffer register TXBUF which holds the latest written data will be returned. As the data is shifted out from either the MSB or the LSB of Transmit Shift Register depending upon SHIFTDIR bit (SPIFMTx.20). Simultaneously, the received bit will be shifted into the Receive Shift Register either through LSB or MSB depending upon SHIFTDIR bit. This allows the concurrent transmission and reception of data. Note: Irrespective of the character length, the Transmit data should be right justified before writing to SPIDAT0 register. The default Data Format Control register for SPIDAT0 is SPIFMT0. However it is possible to reprogram the DFSEL\\[1:0\\]
fields of SPIDAT1 prior to using SPIDAT0, to select a different SPIFMTx register."]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TxdataW<Spidat0Spec> {
        TxdataW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Spidat0Spec> {
        NuW::new(self, 16)
    }
}
#[doc = "SPI / MibSPI Transmit Data Register 0 Note: Accessibility of SPIDAT0 The SPIDAT0 register is not accessible in Multibuffer Mode of MibSPI. It is only accessible in compatibility mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`spidat0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spidat0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spidat0Spec;
impl crate::RegisterSpec for Spidat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spidat0::R`](R) reader structure"]
impl crate::Readable for Spidat0Spec {}
#[doc = "`write(|w| ..)` method takes [`spidat0::W`](W) writer structure"]
impl crate::Writable for Spidat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPIDAT0 to value 0"]
impl crate::Resettable for Spidat0Spec {
    const RESET_VALUE: u32 = 0;
}

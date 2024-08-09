#[doc = "Register `EXTENDED_PRESCALE2` reader"]
pub type R = crate::R<ExtendedPrescale2Spec>;
#[doc = "Register `EXTENDED_PRESCALE2` writer"]
pub type W = crate::W<ExtendedPrescale2Spec>;
#[doc = "Field `EPRESCLAE_FMT2` reader - 10:0\\]
EPRESCALE_FMT2 can be modified in privilege mode only. EPRESCALE_FMT2 determines the bit transfer rate of Data Format 2 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT2(7:0) bits and PRESCALE2(7:0) bits of SPIFMT2 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT3(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EXTENDED_PRESCALE2(26:16) or SPIFMT2(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE2 register is programmed after SPIFMT2 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE2 register when a prescale of more 256 is intended on SPICLK."]
pub type EpresclaeFmt2R = crate::FieldReader<u16>;
#[doc = "Field `EPRESCLAE_FMT2` writer - 10:0\\]
EPRESCALE_FMT2 can be modified in privilege mode only. EPRESCALE_FMT2 determines the bit transfer rate of Data Format 2 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT2(7:0) bits and PRESCALE2(7:0) bits of SPIFMT2 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT3(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EXTENDED_PRESCALE2(26:16) or SPIFMT2(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE2 register is programmed after SPIFMT2 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE2 register when a prescale of more 256 is intended on SPICLK."]
pub type EpresclaeFmt2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU3` reader - 15:11\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 15:11\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EPRESCLAE_FMT3` reader - 26:16\\]
EPRESCALE_FMT3 can be modified in privilege mode only. EPRESCALE_FMT3 determines the bit transfer rate of Data Format 3 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT3(7:0) bits and PRESCALE3(7:0) bits of SPIFMT3 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT3(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EPRESCALE3(26:16) or SPIFMT3(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE2 register is programmed after SPIFMT3 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE2 register when a prescale of more 256 is intended on SPICLK."]
pub type EpresclaeFmt3R = crate::FieldReader<u16>;
#[doc = "Field `EPRESCLAE_FMT3` writer - 26:16\\]
EPRESCALE_FMT3 can be modified in privilege mode only. EPRESCALE_FMT3 determines the bit transfer rate of Data Format 3 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT3(7:0) bits and PRESCALE3(7:0) bits of SPIFMT3 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT3(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EPRESCALE3(26:16) or SPIFMT3(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE2 register is programmed after SPIFMT3 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE2 register when a prescale of more 256 is intended on SPICLK."]
pub type EpresclaeFmt3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU4` reader - 31:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4R = crate::FieldReader;
#[doc = "Field `NU4` writer - 31:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
EPRESCALE_FMT2 can be modified in privilege mode only. EPRESCALE_FMT2 determines the bit transfer rate of Data Format 2 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT2(7:0) bits and PRESCALE2(7:0) bits of SPIFMT2 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT3(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EXTENDED_PRESCALE2(26:16) or SPIFMT2(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE2 register is programmed after SPIFMT2 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE2 register when a prescale of more 256 is intended on SPICLK."]
    #[inline(always)]
    pub fn epresclae_fmt2(&self) -> EpresclaeFmt2R {
        EpresclaeFmt2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
EPRESCALE_FMT3 can be modified in privilege mode only. EPRESCALE_FMT3 determines the bit transfer rate of Data Format 3 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT3(7:0) bits and PRESCALE3(7:0) bits of SPIFMT3 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT3(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EPRESCALE3(26:16) or SPIFMT3(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE2 register is programmed after SPIFMT3 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE2 register when a prescale of more 256 is intended on SPICLK."]
    #[inline(always)]
    pub fn epresclae_fmt3(&self) -> EpresclaeFmt3R {
        EpresclaeFmt3R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
EPRESCALE_FMT2 can be modified in privilege mode only. EPRESCALE_FMT2 determines the bit transfer rate of Data Format 2 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT2(7:0) bits and PRESCALE2(7:0) bits of SPIFMT2 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT3(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EXTENDED_PRESCALE2(26:16) or SPIFMT2(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE2 register is programmed after SPIFMT2 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE2 register when a prescale of more 256 is intended on SPICLK."]
    #[inline(always)]
    #[must_use]
    pub fn epresclae_fmt2(&mut self) -> EpresclaeFmt2W<ExtendedPrescale2Spec> {
        EpresclaeFmt2W::new(self, 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<ExtendedPrescale2Spec> {
        Nu3W::new(self, 11)
    }
    #[doc = "Bits 16:26 - 26:16\\]
EPRESCALE_FMT3 can be modified in privilege mode only. EPRESCALE_FMT3 determines the bit transfer rate of Data Format 3 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT3(7:0) bits and PRESCALE3(7:0) bits of SPIFMT3 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT3(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EPRESCALE3(26:16) or SPIFMT3(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE2 register is programmed after SPIFMT3 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE2 register when a prescale of more 256 is intended on SPICLK."]
    #[inline(always)]
    #[must_use]
    pub fn epresclae_fmt3(&mut self) -> EpresclaeFmt3W<ExtendedPrescale2Spec> {
        EpresclaeFmt3W::new(self, 16)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<ExtendedPrescale2Spec> {
        Nu4W::new(self, 27)
    }
}
#[doc = "SPI/MibSPI Extended Prescale Register 2 (EXTENDED_PRESCALE2 for SPIFMT2 and SPIFMT3) This register provides an extended Prescale values for SPICLK generation to be able to interface with much slower SPI Slaves. This register is an extension of SPIFMT2 and SPIFMT3 registers. For example, EPRESCALE_FMT2(7:0) of EXTENDED_PRESCALE2 and PRESCALE2(7:0) of SPIFMT2 register will always reflect the same contents. Similarly EPRESCALE_FMT3(7:0) and PRESCALE3(7:0) of SPIFMT3 reflect the same contents.\n\nYou can [`read`](crate::Reg::read) this register and get [`extended_prescale2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extended_prescale2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtendedPrescale2Spec;
impl crate::RegisterSpec for ExtendedPrescale2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extended_prescale2::R`](R) reader structure"]
impl crate::Readable for ExtendedPrescale2Spec {}
#[doc = "`write(|w| ..)` method takes [`extended_prescale2::W`](W) writer structure"]
impl crate::Writable for ExtendedPrescale2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTENDED_PRESCALE2 to value 0"]
impl crate::Resettable for ExtendedPrescale2Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `EXTENDED_PRESCALE1` reader"]
pub type R = crate::R<ExtendedPrescale1Spec>;
#[doc = "Register `EXTENDED_PRESCALE1` writer"]
pub type W = crate::W<ExtendedPrescale1Spec>;
#[doc = "Field `EPRESCLAE_FMT0` reader - 10:0\\]
EPRESCALE_FMT0 can be modified in privilege mode only. EPRESCALE_FMT0 determines the bit transfer rate of Data Format 0 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT0(7:0) bits and PRESCALE0(7:0) bits of SPIFMT0 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT0(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EPRESCALE0(26:16) or SPIFMT0(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE1 register is programmed after SPIFMT0 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE1 register when a prescale of more 256 is intended on SPICLK."]
pub type EpresclaeFmt0R = crate::FieldReader<u16>;
#[doc = "Field `EPRESCLAE_FMT0` writer - 10:0\\]
EPRESCALE_FMT0 can be modified in privilege mode only. EPRESCALE_FMT0 determines the bit transfer rate of Data Format 0 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT0(7:0) bits and PRESCALE0(7:0) bits of SPIFMT0 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT0(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EPRESCALE0(26:16) or SPIFMT0(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE1 register is programmed after SPIFMT0 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE1 register when a prescale of more 256 is intended on SPICLK."]
pub type EpresclaeFmt0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU1` reader - 15:11\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 15:11\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EPRESCLAE_FMT1` reader - 26:16\\]
Extended Prescale value for SPIFMT1. EPRESCALE_FMT1 can be modified in privilege mode only. EPRESCALE_FMT1 determines the bit transfer rate of Data Format 1 if the SPI/MibSPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT1(7:0) bits and PRESCALE1(7:0) bits of SPIFMT1 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT1(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EXTENDED_PRESCALE1(26:16) or SPIFMT1(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE1 register is programmed after SPIFMT1 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE1 register when a prescale of more 256 is intended on SPICLK. BRFormatx = VBUSPCLK/(EXTENDEDPRESCALEy+1) When EPRESCALE_FMTy (y=1,2) is set to zero(0), the SPI clock rate defaults to VBUSPCLK/2."]
pub type EpresclaeFmt1R = crate::FieldReader<u16>;
#[doc = "Field `EPRESCLAE_FMT1` writer - 26:16\\]
Extended Prescale value for SPIFMT1. EPRESCALE_FMT1 can be modified in privilege mode only. EPRESCALE_FMT1 determines the bit transfer rate of Data Format 1 if the SPI/MibSPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT1(7:0) bits and PRESCALE1(7:0) bits of SPIFMT1 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT1(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EXTENDED_PRESCALE1(26:16) or SPIFMT1(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE1 register is programmed after SPIFMT1 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE1 register when a prescale of more 256 is intended on SPICLK. BRFormatx = VBUSPCLK/(EXTENDEDPRESCALEy+1) When EPRESCALE_FMTy (y=1,2) is set to zero(0), the SPI clock rate defaults to VBUSPCLK/2."]
pub type EpresclaeFmt1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU2` reader - 31:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 31:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
EPRESCALE_FMT0 can be modified in privilege mode only. EPRESCALE_FMT0 determines the bit transfer rate of Data Format 0 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT0(7:0) bits and PRESCALE0(7:0) bits of SPIFMT0 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT0(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EPRESCALE0(26:16) or SPIFMT0(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE1 register is programmed after SPIFMT0 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE1 register when a prescale of more 256 is intended on SPICLK."]
    #[inline(always)]
    pub fn epresclae_fmt0(&self) -> EpresclaeFmt0R {
        EpresclaeFmt0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Extended Prescale value for SPIFMT1. EPRESCALE_FMT1 can be modified in privilege mode only. EPRESCALE_FMT1 determines the bit transfer rate of Data Format 1 if the SPI/MibSPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT1(7:0) bits and PRESCALE1(7:0) bits of SPIFMT1 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT1(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EXTENDED_PRESCALE1(26:16) or SPIFMT1(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE1 register is programmed after SPIFMT1 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE1 register when a prescale of more 256 is intended on SPICLK. BRFormatx = VBUSPCLK/(EXTENDEDPRESCALEy+1) When EPRESCALE_FMTy (y=1,2) is set to zero(0), the SPI clock rate defaults to VBUSPCLK/2."]
    #[inline(always)]
    pub fn epresclae_fmt1(&self) -> EpresclaeFmt1R {
        EpresclaeFmt1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
EPRESCALE_FMT0 can be modified in privilege mode only. EPRESCALE_FMT0 determines the bit transfer rate of Data Format 0 if the SPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT0(7:0) bits and PRESCALE0(7:0) bits of SPIFMT0 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT0(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EPRESCALE0(26:16) or SPIFMT0(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE1 register is programmed after SPIFMT0 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE1 register when a prescale of more 256 is intended on SPICLK."]
    #[inline(always)]
    #[must_use]
    pub fn epresclae_fmt0(&mut self) -> EpresclaeFmt0W<ExtendedPrescale1Spec> {
        EpresclaeFmt0W::new(self, 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<ExtendedPrescale1Spec> {
        Nu1W::new(self, 11)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Extended Prescale value for SPIFMT1. EPRESCALE_FMT1 can be modified in privilege mode only. EPRESCALE_FMT1 determines the bit transfer rate of Data Format 1 if the SPI/MibSPI is the network master. If the SPI / MibSPI is configured as slave, this field DOES NOT NEED to be configured. These EPRESCALE_FMT1(7:0) bits and PRESCALE1(7:0) bits of SPIFMT1 register will point to the same physically implemented register. Refer to Figure 56 for a graphical representation of the implementation. Write : This register field should be written if a SPICLK prescaler of more VBUSPCLK/256 is required. This field provides a prescaler of up to VBUSPCLK/2048 for SPICLK. Writing to this register field will also get reflected in SPIFMT1(15:8). Read : Reading this field will reflect the PRESCALE value based on the last written register field i.e., EXTENDED_PRESCALE1(26:16) or SPIFMT1(15:8) register. Note: If Extended Prescaler is required, it should be ensured that EXTENDED_PRESCALE1 register is programmed after SPIFMT1 register is programmed. This is to ensure that the final SPICLK prescale value is controlled by EXTENDED_PRESCALE1 register when a prescale of more 256 is intended on SPICLK. BRFormatx = VBUSPCLK/(EXTENDEDPRESCALEy+1) When EPRESCALE_FMTy (y=1,2) is set to zero(0), the SPI clock rate defaults to VBUSPCLK/2."]
    #[inline(always)]
    #[must_use]
    pub fn epresclae_fmt1(&mut self) -> EpresclaeFmt1W<ExtendedPrescale1Spec> {
        EpresclaeFmt1W::new(self, 16)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved. Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<ExtendedPrescale1Spec> {
        Nu2W::new(self, 27)
    }
}
#[doc = "SPI/MibSPI Extended Prescale Register 1 (EXTENDED_PRESCALE1 for SPIFMT0 and SPIFMT1) This register provides an extended Prescale values for SPICLK generation to be able to interface with much slower SPI Slaves. This is an extension of SPIFMT0 and SPIFMT1 registers. For example, EPRESCALE_FMT1(7:0) of EXTENDED_PRESCALE1 and PRESCALE1(7:0) of SPIFMT1 register will always reflect the same contents. Similarly EPRESCALE_FMT0(7:0) and PRESCALE0(7:0) of SPIFMT0 reflect the same contents.\n\nYou can [`read`](crate::Reg::read) this register and get [`extended_prescale1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extended_prescale1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtendedPrescale1Spec;
impl crate::RegisterSpec for ExtendedPrescale1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extended_prescale1::R`](R) reader structure"]
impl crate::Readable for ExtendedPrescale1Spec {}
#[doc = "`write(|w| ..)` method takes [`extended_prescale1::W`](W) writer structure"]
impl crate::Writable for ExtendedPrescale1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTENDED_PRESCALE1 to value 0"]
impl crate::Resettable for ExtendedPrescale1Spec {
    const RESET_VALUE: u32 = 0;
}

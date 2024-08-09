#[doc = "Register `RS232_SOP11_BITINTERVAL` reader"]
pub type R = crate::R<Rs232Sop11BitintervalSpec>;
#[doc = "Register `RS232_SOP11_BITINTERVAL` writer"]
pub type W = crate::W<Rs232Sop11BitintervalSpec>;
#[doc = "Field `val` reader - 31:0\\]
RS232 Bit Interval when SOP11 (ANA_WU_MODE_REG_LOWV\\[6:5\\]) is selected. XTAL is 50M and this field can be used to modify the baud rate."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - 31:0\\]
RS232 Bit Interval when SOP11 (ANA_WU_MODE_REG_LOWV\\[6:5\\]) is selected. XTAL is 50M and this field can be used to modify the baud rate."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
RS232 Bit Interval when SOP11 (ANA_WU_MODE_REG_LOWV\\[6:5\\]) is selected. XTAL is 50M and this field can be used to modify the baud rate."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
RS232 Bit Interval when SOP11 (ANA_WU_MODE_REG_LOWV\\[6:5\\]) is selected. XTAL is 50M and this field can be used to modify the baud rate."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Rs232Sop11BitintervalSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "RS232_SOP11_BITINTERVAL\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_sop11_bitinterval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_sop11_bitinterval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs232Sop11BitintervalSpec;
impl crate::RegisterSpec for Rs232Sop11BitintervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs232_sop11_bitinterval::R`](R) reader structure"]
impl crate::Readable for Rs232Sop11BitintervalSpec {}
#[doc = "`write(|w| ..)` method takes [`rs232_sop11_bitinterval::W`](W) writer structure"]
impl crate::Writable for Rs232Sop11BitintervalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS232_SOP11_BITINTERVAL to value 0"]
impl crate::Resettable for Rs232Sop11BitintervalSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CCC_ERR_STATUS` reader"]
pub type R = crate::R<CccErrStatusSpec>;
#[doc = "Register `CCC_ERR_STATUS` writer"]
pub type W = crate::W<CccErrStatusSpec>;
#[doc = "Field `ccca_errot_status` reader - 7:0\\]
CCCA Error Status (for Debug) {3'd0, counter_error, counter_done, timeout_error, counter_error, counter_done}"]
pub type CccaErrotStatusR = crate::FieldReader;
#[doc = "Field `ccca_errot_status` writer - 7:0\\]
CCCA Error Status (for Debug) {3'd0, counter_error, counter_done, timeout_error, counter_error, counter_done}"]
pub type CccaErrotStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cccb_errot_status` reader - 23:16\\]
CCCB Error Status (for Debug) {3'd0, counter_error, counter_done, timeout_error, counter_error, counter_done}"]
pub type CccbErrotStatusR = crate::FieldReader;
#[doc = "Field `cccb_errot_status` writer - 23:16\\]
CCCB Error Status (for Debug) {3'd0, counter_error, counter_done, timeout_error, counter_error, counter_done}"]
pub type CccbErrotStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
CCCA Error Status (for Debug) {3'd0, counter_error, counter_done, timeout_error, counter_error, counter_done}"]
    #[inline(always)]
    pub fn ccca_errot_status(&self) -> CccaErrotStatusR {
        CccaErrotStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
CCCB Error Status (for Debug) {3'd0, counter_error, counter_done, timeout_error, counter_error, counter_done}"]
    #[inline(always)]
    pub fn cccb_errot_status(&self) -> CccbErrotStatusR {
        CccbErrotStatusR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
CCCA Error Status (for Debug) {3'd0, counter_error, counter_done, timeout_error, counter_error, counter_done}"]
    #[inline(always)]
    #[must_use]
    pub fn ccca_errot_status(&mut self) -> CccaErrotStatusW<CccErrStatusSpec> {
        CccaErrotStatusW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
CCCB Error Status (for Debug) {3'd0, counter_error, counter_done, timeout_error, counter_error, counter_done}"]
    #[inline(always)]
    #[must_use]
    pub fn cccb_errot_status(&mut self) -> CccbErrotStatusW<CccErrStatusSpec> {
        CccbErrotStatusW::new(self, 16)
    }
}
#[doc = "CCC_ERR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`ccc_err_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccc_err_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccErrStatusSpec;
impl crate::RegisterSpec for CccErrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccc_err_status::R`](R) reader structure"]
impl crate::Readable for CccErrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ccc_err_status::W`](W) writer structure"]
impl crate::Writable for CccErrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCC_ERR_STATUS to value 0"]
impl crate::Resettable for CccErrStatusSpec {
    const RESET_VALUE: u32 = 0;
}

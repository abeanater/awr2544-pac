#[doc = "Register `MSS_STC_CONTROL` reader"]
pub type R = crate::R<MssStcControlSpec>;
#[doc = "Register `MSS_STC_CONTROL` writer"]
pub type W = crate::W<MssStcControlSpec>;
#[doc = "Field `cr5_wfi_overide` reader - 2:0\\]
writing 3'b111 will bypass the wfi signals from R5SS."]
pub type Cr5WfiOverideR = crate::FieldReader;
#[doc = "Field `cr5_wfi_overide` writer - 2:0\\]
writing 3'b111 will bypass the wfi signals from R5SS."]
pub type Cr5WfiOverideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing 3'b111 will bypass the wfi signals from R5SS."]
    #[inline(always)]
    pub fn cr5_wfi_overide(&self) -> Cr5WfiOverideR {
        Cr5WfiOverideR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing 3'b111 will bypass the wfi signals from R5SS."]
    #[inline(always)]
    #[must_use]
    pub fn cr5_wfi_overide(&mut self) -> Cr5WfiOverideW<MssStcControlSpec> {
        Cr5WfiOverideW::new(self, 0)
    }
}
#[doc = "MSS_STC_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_stc_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_stc_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssStcControlSpec;
impl crate::RegisterSpec for MssStcControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_stc_control::R`](R) reader structure"]
impl crate::Readable for MssStcControlSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_stc_control::W`](W) writer structure"]
impl crate::Writable for MssStcControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_STC_CONTROL to value 0"]
impl crate::Resettable for MssStcControlSpec {
    const RESET_VALUE: u32 = 0;
}

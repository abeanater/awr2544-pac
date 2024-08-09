#[doc = "Register `DC_EST_I_7_VAL` reader"]
pub type R = crate::R<DcEstI7ValSpec>;
#[doc = "Register `DC_EST_I_7_VAL` writer"]
pub type W = crate::W<DcEstI7ValSpec>;
#[doc = "Field `dc_est_i_7_val` reader - 23:0\\]
This read only register provide the DC estimates I for bcnt= 7"]
pub type DcEstI7ValR = crate::FieldReader<u32>;
#[doc = "Field `dc_est_i_7_val` writer - 23:0\\]
This read only register provide the DC estimates I for bcnt= 7"]
pub type DcEstI7ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This read only register provide the DC estimates I for bcnt= 7"]
    #[inline(always)]
    pub fn dc_est_i_7_val(&self) -> DcEstI7ValR {
        DcEstI7ValR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This read only register provide the DC estimates I for bcnt= 7"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_i_7_val(&mut self) -> DcEstI7ValW<DcEstI7ValSpec> {
        DcEstI7ValW::new(self, 0)
    }
}
#[doc = "DC_EST_I_7_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_7_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_7_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcEstI7ValSpec;
impl crate::RegisterSpec for DcEstI7ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_est_i_7_val::R`](R) reader structure"]
impl crate::Readable for DcEstI7ValSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_est_i_7_val::W`](W) writer structure"]
impl crate::Writable for DcEstI7ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_EST_I_7_VAL to value 0"]
impl crate::Resettable for DcEstI7ValSpec {
    const RESET_VALUE: u32 = 0;
}

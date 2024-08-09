#[doc = "Register `DC_EST_I_5_VAL` reader"]
pub type R = crate::R<DcEstI5ValSpec>;
#[doc = "Register `DC_EST_I_5_VAL` writer"]
pub type W = crate::W<DcEstI5ValSpec>;
#[doc = "Field `dc_est_i_5_val` reader - 23:0\\]
This read only register provide the DC estimates I for bcnt= 5"]
pub type DcEstI5ValR = crate::FieldReader<u32>;
#[doc = "Field `dc_est_i_5_val` writer - 23:0\\]
This read only register provide the DC estimates I for bcnt= 5"]
pub type DcEstI5ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This read only register provide the DC estimates I for bcnt= 5"]
    #[inline(always)]
    pub fn dc_est_i_5_val(&self) -> DcEstI5ValR {
        DcEstI5ValR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This read only register provide the DC estimates I for bcnt= 5"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_i_5_val(&mut self) -> DcEstI5ValW<DcEstI5ValSpec> {
        DcEstI5ValW::new(self, 0)
    }
}
#[doc = "DC_EST_I_5_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_i_5_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_i_5_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcEstI5ValSpec;
impl crate::RegisterSpec for DcEstI5ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_est_i_5_val::R`](R) reader structure"]
impl crate::Readable for DcEstI5ValSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_est_i_5_val::W`](W) writer structure"]
impl crate::Writable for DcEstI5ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_EST_I_5_VAL to value 0"]
impl crate::Resettable for DcEstI5ValSpec {
    const RESET_VALUE: u32 = 0;
}

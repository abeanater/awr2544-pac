#[doc = "Register `DC_EST_Q_2_VAL` reader"]
pub type R = crate::R<DcEstQ2ValSpec>;
#[doc = "Register `DC_EST_Q_2_VAL` writer"]
pub type W = crate::W<DcEstQ2ValSpec>;
#[doc = "Field `dc_est_q_2_val` reader - 23:0\\]
This read only register provide the DC estimates Q for bcnt= 2"]
pub type DcEstQ2ValR = crate::FieldReader<u32>;
#[doc = "Field `dc_est_q_2_val` writer - 23:0\\]
This read only register provide the DC estimates Q for bcnt= 2"]
pub type DcEstQ2ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This read only register provide the DC estimates Q for bcnt= 2"]
    #[inline(always)]
    pub fn dc_est_q_2_val(&self) -> DcEstQ2ValR {
        DcEstQ2ValR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This read only register provide the DC estimates Q for bcnt= 2"]
    #[inline(always)]
    #[must_use]
    pub fn dc_est_q_2_val(&mut self) -> DcEstQ2ValW<DcEstQ2ValSpec> {
        DcEstQ2ValW::new(self, 0)
    }
}
#[doc = "DC_EST_Q_2_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_est_q_2_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_est_q_2_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcEstQ2ValSpec;
impl crate::RegisterSpec for DcEstQ2ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_est_q_2_val::R`](R) reader structure"]
impl crate::Readable for DcEstQ2ValSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_est_q_2_val::W`](W) writer structure"]
impl crate::Writable for DcEstQ2ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_EST_Q_2_VAL to value 0"]
impl crate::Resettable for DcEstQ2ValSpec {
    const RESET_VALUE: u32 = 0;
}

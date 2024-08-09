#[doc = "Register `QCMULT_SCALE1` reader"]
pub type R = crate::R<QcmultScale1Spec>;
#[doc = "Register `QCMULT_SCALE1` writer"]
pub type W = crate::W<QcmultScale1Spec>;
#[doc = "Field `qcmult_scale1` reader - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type QcmultScale1R = crate::FieldReader<u32>;
#[doc = "Field `qcmult_scale1` writer - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type QcmultScale1W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    pub fn qcmult_scale1(&self) -> QcmultScale1R {
        QcmultScale1R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    #[must_use]
    pub fn qcmult_scale1(&mut self) -> QcmultScale1W<QcmultScale1Spec> {
        QcmultScale1W::new(self, 0)
    }
}
#[doc = "QCMULT_SCALE1\n\nYou can [`read`](crate::Reg::read) this register and get [`qcmult_scale1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcmult_scale1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcmultScale1Spec;
impl crate::RegisterSpec for QcmultScale1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qcmult_scale1::R`](R) reader structure"]
impl crate::Readable for QcmultScale1Spec {}
#[doc = "`write(|w| ..)` method takes [`qcmult_scale1::W`](W) writer structure"]
impl crate::Writable for QcmultScale1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCMULT_SCALE1 to value 0"]
impl crate::Resettable for QcmultScale1Spec {
    const RESET_VALUE: u32 = 0;
}

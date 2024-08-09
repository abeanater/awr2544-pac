#[doc = "Register `QCMULT_SCALE3` reader"]
pub type R = crate::R<QcmultScale3Spec>;
#[doc = "Register `QCMULT_SCALE3` writer"]
pub type W = crate::W<QcmultScale3Spec>;
#[doc = "Field `qcmult_scale3` reader - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type QcmultScale3R = crate::FieldReader<u32>;
#[doc = "Field `qcmult_scale3` writer - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type QcmultScale3W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    pub fn qcmult_scale3(&self) -> QcmultScale3R {
        QcmultScale3R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    #[must_use]
    pub fn qcmult_scale3(&mut self) -> QcmultScale3W<QcmultScale3Spec> {
        QcmultScale3W::new(self, 0)
    }
}
#[doc = "QCMULT_SCALE3\n\nYou can [`read`](crate::Reg::read) this register and get [`qcmult_scale3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcmult_scale3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcmultScale3Spec;
impl crate::RegisterSpec for QcmultScale3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qcmult_scale3::R`](R) reader structure"]
impl crate::Readable for QcmultScale3Spec {}
#[doc = "`write(|w| ..)` method takes [`qcmult_scale3::W`](W) writer structure"]
impl crate::Writable for QcmultScale3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCMULT_SCALE3 to value 0"]
impl crate::Resettable for QcmultScale3Spec {
    const RESET_VALUE: u32 = 0;
}

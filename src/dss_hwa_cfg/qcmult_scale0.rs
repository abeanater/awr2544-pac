#[doc = "Register `QCMULT_SCALE0` reader"]
pub type R = crate::R<QcmultScale0Spec>;
#[doc = "Register `QCMULT_SCALE0` writer"]
pub type W = crate::W<QcmultScale0Spec>;
#[doc = "Field `qcmult_scale0` reader - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type QcmultScale0R = crate::FieldReader<u32>;
#[doc = "Field `qcmult_scale0` writer - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type QcmultScale0W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    pub fn qcmult_scale0(&self) -> QcmultScale0R {
        QcmultScale0R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    #[must_use]
    pub fn qcmult_scale0(&mut self) -> QcmultScale0W<QcmultScale0Spec> {
        QcmultScale0W::new(self, 0)
    }
}
#[doc = "QCMULT_SCALE0\n\nYou can [`read`](crate::Reg::read) this register and get [`qcmult_scale0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcmult_scale0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcmultScale0Spec;
impl crate::RegisterSpec for QcmultScale0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qcmult_scale0::R`](R) reader structure"]
impl crate::Readable for QcmultScale0Spec {}
#[doc = "`write(|w| ..)` method takes [`qcmult_scale0::W`](W) writer structure"]
impl crate::Writable for QcmultScale0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCMULT_SCALE0 to value 0"]
impl crate::Resettable for QcmultScale0Spec {
    const RESET_VALUE: u32 = 0;
}

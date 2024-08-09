#[doc = "Register `QCMULT_SCALE2` reader"]
pub type R = crate::R<QcmultScale2Spec>;
#[doc = "Register `QCMULT_SCALE2` writer"]
pub type W = crate::W<QcmultScale2Spec>;
#[doc = "Field `qcmult_scale2` reader - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type QcmultScale2R = crate::FieldReader<u32>;
#[doc = "Field `qcmult_scale2` writer - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type QcmultScale2W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    pub fn qcmult_scale2(&self) -> QcmultScale2R {
        QcmultScale2R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    #[must_use]
    pub fn qcmult_scale2(&mut self) -> QcmultScale2W<QcmultScale2Spec> {
        QcmultScale2W::new(self, 0)
    }
}
#[doc = "QCMULT_SCALE2\n\nYou can [`read`](crate::Reg::read) this register and get [`qcmult_scale2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcmult_scale2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QcmultScale2Spec;
impl crate::RegisterSpec for QcmultScale2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qcmult_scale2::R`](R) reader structure"]
impl crate::Readable for QcmultScale2Spec {}
#[doc = "`write(|w| ..)` method takes [`qcmult_scale2::W`](W) writer structure"]
impl crate::Writable for QcmultScale2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCMULT_SCALE2 to value 0"]
impl crate::Resettable for QcmultScale2Spec {
    const RESET_VALUE: u32 = 0;
}

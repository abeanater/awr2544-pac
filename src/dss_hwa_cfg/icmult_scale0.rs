#[doc = "Register `ICMULT_SCALE0` reader"]
pub type R = crate::R<IcmultScale0Spec>;
#[doc = "Register `ICMULT_SCALE0` writer"]
pub type W = crate::W<IcmultScale0Spec>;
#[doc = "Field `icmult_scale0` reader - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type IcmultScale0R = crate::FieldReader<u32>;
#[doc = "Field `icmult_scale0` writer - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
pub type IcmultScale0W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    pub fn icmult_scale0(&self) -> IcmultScale0R {
        IcmultScale0R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Complex scalars used in CMULT_MODE = 0101, In CMULT_MODE : 0101 , The input samples are multiplied by a different complex scalar per-iteration based on REG_BCNT in complex inputs mode."]
    #[inline(always)]
    #[must_use]
    pub fn icmult_scale0(&mut self) -> IcmultScale0W<IcmultScale0Spec> {
        IcmultScale0W::new(self, 0)
    }
}
#[doc = "ICMULT_SCALE0\n\nYou can [`read`](crate::Reg::read) this register and get [`icmult_scale0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icmult_scale0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcmultScale0Spec;
impl crate::RegisterSpec for IcmultScale0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icmult_scale0::R`](R) reader structure"]
impl crate::Readable for IcmultScale0Spec {}
#[doc = "`write(|w| ..)` method takes [`icmult_scale0::W`](W) writer structure"]
impl crate::Writable for IcmultScale0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICMULT_SCALE0 to value 0"]
impl crate::Resettable for IcmultScale0Spec {
    const RESET_VALUE: u32 = 0;
}

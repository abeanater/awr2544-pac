#[doc = "Register `CH_HIL_CP_OVERRIDE` reader"]
pub type R = crate::R<ChHilCpOverrideSpec>;
#[doc = "Register `CH_HIL_CP_OVERRIDE` writer"]
pub type W = crate::W<ChHilCpOverrideSpec>;
#[doc = "Field `chirp` reader - 15:0\\]
Override data used for Chirp. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type ChirpR = crate::FieldReader<u16>;
#[doc = "Field `chirp` writer - 15:0\\]
Override data used for Chirp. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
pub type ChirpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    pub fn chirp(&self) -> ChirpR {
        ChirpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Override data used for Chirp. data\\[11:0\\]
is used for overriding chirp number data\\[15:12\\]
is used for overriding chirp profile index"]
    #[inline(always)]
    #[must_use]
    pub fn chirp(&mut self) -> ChirpW<ChHilCpOverrideSpec> {
        ChirpW::new(self, 0)
    }
}
#[doc = "CH_HIL_CP_OVERRIDE\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_hil_cp_override::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_hil_cp_override::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChHilCpOverrideSpec;
impl crate::RegisterSpec for ChHilCpOverrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_hil_cp_override::R`](R) reader structure"]
impl crate::Readable for ChHilCpOverrideSpec {}
#[doc = "`write(|w| ..)` method takes [`ch_hil_cp_override::W`](W) writer structure"]
impl crate::Writable for ChHilCpOverrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_HIL_CP_OVERRIDE to value 0"]
impl crate::Resettable for ChHilCpOverrideSpec {
    const RESET_VALUE: u32 = 0;
}

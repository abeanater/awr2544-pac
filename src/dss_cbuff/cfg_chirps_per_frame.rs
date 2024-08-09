#[doc = "Register `CFG_CHIRPS_PER_FRAME` reader"]
pub type R = crate::R<CfgChirpsPerFrameSpec>;
#[doc = "Register `CFG_CHIRPS_PER_FRAME` writer"]
pub type W = crate::W<CfgChirpsPerFrameSpec>;
#[doc = "Field `CFG_CHIRPS_PER_FRAME` reader - 31:0\\]
Configure the number of Chirps in a Frame"]
pub type CfgChirpsPerFrameR = crate::FieldReader<u32>;
#[doc = "Field `CFG_CHIRPS_PER_FRAME` writer - 31:0\\]
Configure the number of Chirps in a Frame"]
pub type CfgChirpsPerFrameW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Configure the number of Chirps in a Frame"]
    #[inline(always)]
    pub fn cfg_chirps_per_frame(&self) -> CfgChirpsPerFrameR {
        CfgChirpsPerFrameR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Configure the number of Chirps in a Frame"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_chirps_per_frame(&mut self) -> CfgChirpsPerFrameW<CfgChirpsPerFrameSpec> {
        CfgChirpsPerFrameW::new(self, 0)
    }
}
#[doc = "Number of Chirps per Frame\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_chirps_per_frame::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_chirps_per_frame::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgChirpsPerFrameSpec;
impl crate::RegisterSpec for CfgChirpsPerFrameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_chirps_per_frame::R`](R) reader structure"]
impl crate::Readable for CfgChirpsPerFrameSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_chirps_per_frame::W`](W) writer structure"]
impl crate::Writable for CfgChirpsPerFrameSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CHIRPS_PER_FRAME to value 0"]
impl crate::Resettable for CfgChirpsPerFrameSpec {
    const RESET_VALUE: u32 = 0;
}

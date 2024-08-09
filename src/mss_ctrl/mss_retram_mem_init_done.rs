#[doc = "Register `MSS_RETRAM_MEM_INIT_DONE` reader"]
pub type R = crate::R<MssRetramMemInitDoneSpec>;
#[doc = "Register `MSS_RETRAM_MEM_INIT_DONE` writer"]
pub type W = crate::W<MssRetramMemInitDoneSpec>;
#[doc = "Field `mem0_done` reader - 0:0\\]
This field will be high once intialization of MSS_RETRAM is finished. Writing '1' would clear the bit"]
pub type Mem0DoneR = crate::BitReader;
#[doc = "Field `mem0_done` writer - 0:0\\]
This field will be high once intialization of MSS_RETRAM is finished. Writing '1' would clear the bit"]
pub type Mem0DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once intialization of MSS_RETRAM is finished. Writing '1' would clear the bit"]
    #[inline(always)]
    pub fn mem0_done(&self) -> Mem0DoneR {
        Mem0DoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This field will be high once intialization of MSS_RETRAM is finished. Writing '1' would clear the bit"]
    #[inline(always)]
    #[must_use]
    pub fn mem0_done(&mut self) -> Mem0DoneW<MssRetramMemInitDoneSpec> {
        Mem0DoneW::new(self, 0)
    }
}
#[doc = "MSS_RETRAM_MEM_INIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_retram_mem_init_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_retram_mem_init_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssRetramMemInitDoneSpec;
impl crate::RegisterSpec for MssRetramMemInitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_retram_mem_init_done::R`](R) reader structure"]
impl crate::Readable for MssRetramMemInitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_retram_mem_init_done::W`](W) writer structure"]
impl crate::Writable for MssRetramMemInitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_RETRAM_MEM_INIT_DONE to value 0"]
impl crate::Resettable for MssRetramMemInitDoneSpec {
    const RESET_VALUE: u32 = 0;
}

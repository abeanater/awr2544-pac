#[doc = "Register `BSS_DFE_MEMINIT_DONE` reader"]
pub type R = crate::R<BssDfeMeminitDoneSpec>;
#[doc = "Register `BSS_DFE_MEMINIT_DONE` writer"]
pub type W = crate::W<BssDfeMeminitDoneSpec>;
#[doc = "Field `mem0_done` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
pub type Mem0DoneR = crate::BitReader;
#[doc = "Field `mem0_done` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
pub type Mem0DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
    #[inline(always)]
    pub fn mem0_done(&self) -> Mem0DoneR {
        Mem0DoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
    #[inline(always)]
    #[must_use]
    pub fn mem0_done(&mut self) -> Mem0DoneW<BssDfeMeminitDoneSpec> {
        Mem0DoneW::new(self, 0)
    }
}
#[doc = "BSS_DFE_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dfe_meminit_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dfe_meminit_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BssDfeMeminitDoneSpec;
impl crate::RegisterSpec for BssDfeMeminitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bss_dfe_meminit_done::R`](R) reader structure"]
impl crate::Readable for BssDfeMeminitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`bss_dfe_meminit_done::W`](W) writer structure"]
impl crate::Writable for BssDfeMeminitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSS_DFE_MEMINIT_DONE to value 0"]
impl crate::Resettable for BssDfeMeminitDoneSpec {
    const RESET_VALUE: u32 = 0;
}

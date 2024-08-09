#[doc = "Register `BSS_TCM_MEMINIT_STATUS` reader"]
pub type R = crate::R<BssTcmMeminitStatusSpec>;
#[doc = "Register `BSS_TCM_MEMINIT_STATUS` writer"]
pub type W = crate::W<BssTcmMeminitStatusSpec>;
#[doc = "Field `mem0_status` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
pub type Mem0StatusR = crate::BitReader;
#[doc = "Field `mem0_status` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
pub type Mem0StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
    #[inline(always)]
    pub fn mem0_status(&self) -> Mem0StatusR {
        Mem0StatusR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn mem0_status(&mut self) -> Mem0StatusW<BssTcmMeminitStatusSpec> {
        Mem0StatusW::new(self, 0)
    }
}
#[doc = "BSS_TCM_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_tcm_meminit_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_tcm_meminit_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BssTcmMeminitStatusSpec;
impl crate::RegisterSpec for BssTcmMeminitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bss_tcm_meminit_status::R`](R) reader structure"]
impl crate::Readable for BssTcmMeminitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`bss_tcm_meminit_status::W`](W) writer structure"]
impl crate::Writable for BssTcmMeminitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSS_TCM_MEMINIT_STATUS to value 0"]
impl crate::Resettable for BssTcmMeminitStatusSpec {
    const RESET_VALUE: u32 = 0;
}

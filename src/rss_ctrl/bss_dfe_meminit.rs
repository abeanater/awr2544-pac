#[doc = "Register `BSS_DFE_MEMINIT` reader"]
pub type R = crate::R<BssDfeMeminitSpec>;
#[doc = "Register `BSS_DFE_MEMINIT` writer"]
pub type W = crate::W<BssDfeMeminitSpec>;
#[doc = "Field `mem0_init` reader - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization."]
pub type Mem0InitR = crate::BitReader;
#[doc = "Field `mem0_init` writer - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization."]
pub type Mem0InitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization."]
    #[inline(always)]
    pub fn mem0_init(&self) -> Mem0InitR {
        Mem0InitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization."]
    #[inline(always)]
    #[must_use]
    pub fn mem0_init(&mut self) -> Mem0InitW<BssDfeMeminitSpec> {
        Mem0InitW::new(self, 0)
    }
}
#[doc = "BSS_DFE_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`bss_dfe_meminit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bss_dfe_meminit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BssDfeMeminitSpec;
impl crate::RegisterSpec for BssDfeMeminitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bss_dfe_meminit::R`](R) reader structure"]
impl crate::Readable for BssDfeMeminitSpec {}
#[doc = "`write(|w| ..)` method takes [`bss_dfe_meminit::W`](W) writer structure"]
impl crate::Writable for BssDfeMeminitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSS_DFE_MEMINIT to value 0"]
impl crate::Resettable for BssDfeMeminitSpec {
    const RESET_VALUE: u32 = 0;
}

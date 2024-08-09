#[doc = "Register `RSS_SHARED_MEM_MEMINIT` reader"]
pub type R = crate::R<RssSharedMemMeminitSpec>;
#[doc = "Register `RSS_SHARED_MEM_MEMINIT` writer"]
pub type W = crate::W<RssSharedMemMeminitSpec>;
#[doc = "Field `start` reader - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed."]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start Memory intialization of memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<RssSharedMemMeminitSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "RSS_SHARED_MEM_MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_shared_mem_meminit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_shared_mem_meminit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssSharedMemMeminitSpec;
impl crate::RegisterSpec for RssSharedMemMeminitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_shared_mem_meminit::R`](R) reader structure"]
impl crate::Readable for RssSharedMemMeminitSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_shared_mem_meminit::W`](W) writer structure"]
impl crate::Writable for RssSharedMemMeminitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_SHARED_MEM_MEMINIT to value 0"]
impl crate::Resettable for RssSharedMemMeminitSpec {
    const RESET_VALUE: u32 = 0;
}

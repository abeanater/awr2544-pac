#[doc = "Register `RSS_ADCBUF_PONG_MEMINIT_DONE` reader"]
pub type R = crate::R<RssAdcbufPongMeminitDoneSpec>;
#[doc = "Register `RSS_ADCBUF_PONG_MEMINIT_DONE` writer"]
pub type W = crate::W<RssAdcbufPongMeminitDoneSpec>;
#[doc = "Field `done` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
pub type DoneR = crate::BitReader;
#[doc = "Field `done` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of memory is complte. Write 0x1 to clear status."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<RssAdcbufPongMeminitDoneSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "RSS_ADCBUF_PONG_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_adcbuf_pong_meminit_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_adcbuf_pong_meminit_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssAdcbufPongMeminitDoneSpec;
impl crate::RegisterSpec for RssAdcbufPongMeminitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_adcbuf_pong_meminit_done::R`](R) reader structure"]
impl crate::Readable for RssAdcbufPongMeminitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_adcbuf_pong_meminit_done::W`](W) writer structure"]
impl crate::Writable for RssAdcbufPongMeminitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_ADCBUF_PONG_MEMINIT_DONE to value 0"]
impl crate::Resettable for RssAdcbufPongMeminitDoneSpec {
    const RESET_VALUE: u32 = 0;
}

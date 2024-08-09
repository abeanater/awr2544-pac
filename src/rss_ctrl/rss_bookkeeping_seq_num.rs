#[doc = "Register `RSS_BOOKKEEPING_SEQ_NUM` reader"]
pub type R = crate::R<RssBookkeepingSeqNumSpec>;
#[doc = "Register `RSS_BOOKKEEPING_SEQ_NUM` writer"]
pub type W = crate::W<RssBookkeepingSeqNumSpec>;
#[doc = "Field `reg` reader - 31:0\\]
SEQ_NUM counter is incremented on every read to this register"]
pub type RegR = crate::FieldReader<u32>;
#[doc = "Field `reg` writer - 31:0\\]
SEQ_NUM counter is incremented on every read to this register"]
pub type RegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
SEQ_NUM counter is incremented on every read to this register"]
    #[inline(always)]
    pub fn reg(&self) -> RegR {
        RegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
SEQ_NUM counter is incremented on every read to this register"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> RegW<RssBookkeepingSeqNumSpec> {
        RegW::new(self, 0)
    }
}
#[doc = "RSS_BOOKKEEPING_SEQ_NUM\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bookkeeping_seq_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bookkeeping_seq_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssBookkeepingSeqNumSpec;
impl crate::RegisterSpec for RssBookkeepingSeqNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_bookkeeping_seq_num::R`](R) reader structure"]
impl crate::Readable for RssBookkeepingSeqNumSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_bookkeeping_seq_num::W`](W) writer structure"]
impl crate::Writable for RssBookkeepingSeqNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_BOOKKEEPING_SEQ_NUM to value 0"]
impl crate::Resettable for RssBookkeepingSeqNumSpec {
    const RESET_VALUE: u32 = 0;
}

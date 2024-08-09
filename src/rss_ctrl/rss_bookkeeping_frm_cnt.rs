#[doc = "Register `RSS_BOOKKEEPING_FRM_CNT` reader"]
pub type R = crate::R<RssBookkeepingFrmCntSpec>;
#[doc = "Register `RSS_BOOKKEEPING_FRM_CNT` writer"]
pub type W = crate::W<RssBookkeepingFrmCntSpec>;
#[doc = "Field `reg` reader - 31:0\\]
FRM_CNT counter is incremented on every frame count trigger source selected using RSS_BOOKKEEPING_CTRL"]
pub type RegR = crate::FieldReader<u32>;
#[doc = "Field `reg` writer - 31:0\\]
FRM_CNT counter is incremented on every frame count trigger source selected using RSS_BOOKKEEPING_CTRL"]
pub type RegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
FRM_CNT counter is incremented on every frame count trigger source selected using RSS_BOOKKEEPING_CTRL"]
    #[inline(always)]
    pub fn reg(&self) -> RegR {
        RegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
FRM_CNT counter is incremented on every frame count trigger source selected using RSS_BOOKKEEPING_CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> RegW<RssBookkeepingFrmCntSpec> {
        RegW::new(self, 0)
    }
}
#[doc = "RSS_BOOKKEEPING_FRM_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bookkeeping_frm_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bookkeeping_frm_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssBookkeepingFrmCntSpec;
impl crate::RegisterSpec for RssBookkeepingFrmCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_bookkeeping_frm_cnt::R`](R) reader structure"]
impl crate::Readable for RssBookkeepingFrmCntSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_bookkeeping_frm_cnt::W`](W) writer structure"]
impl crate::Writable for RssBookkeepingFrmCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_BOOKKEEPING_FRM_CNT to value 0"]
impl crate::Resettable for RssBookkeepingFrmCntSpec {
    const RESET_VALUE: u32 = 0;
}

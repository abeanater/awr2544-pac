#[doc = "Register `RSS_BOOKKEEPING_CTRL` reader"]
pub type R = crate::R<RssBookkeepingCtrlSpec>;
#[doc = "Register `RSS_BOOKKEEPING_CTRL` writer"]
pub type W = crate::W<RssBookkeepingCtrlSpec>;
#[doc = "Field `seq_num_rst` reader - 0:0\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_SEQ_NUM"]
pub type SeqNumRstR = crate::BitReader;
#[doc = "Field `seq_num_rst` writer - 0:0\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_SEQ_NUM"]
pub type SeqNumRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frm_cnt_rst` reader - 4:4\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_FRM_CNT"]
pub type FrmCntRstR = crate::BitReader;
#[doc = "Field `frm_cnt_rst` writer - 4:4\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_FRM_CNT"]
pub type FrmCntRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chrp_cnt_rst` reader - 8:8\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_CHRP_CNT"]
pub type ChrpCntRstR = crate::BitReader;
#[doc = "Field `chrp_cnt_rst` writer - 8:8\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_CHRP_CNT"]
pub type ChrpCntRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frm_cnt_trig_src` reader - 12:12\\]
0x0: RSS_BOOKKEEPING_FRM_CNT is incremented on every FRAME_START 0x1: RSS_BOOKKEEPING_FRM_CNT is incremented on every FRAME_END"]
pub type FrmCntTrigSrcR = crate::BitReader;
#[doc = "Field `frm_cnt_trig_src` writer - 12:12\\]
0x0: RSS_BOOKKEEPING_FRM_CNT is incremented on every FRAME_START 0x1: RSS_BOOKKEEPING_FRM_CNT is incremented on every FRAME_END"]
pub type FrmCntTrigSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chrp_cnt_trig_src` reader - 16:16\\]
0x0: RSS_BOOKKEEPING_CHRP_CNT is incremented on every CHIRP_START 0x1: RSS_BOOKKEEPING_CHRP_CNT is incremented on every CHIRP_END"]
pub type ChrpCntTrigSrcR = crate::BitReader;
#[doc = "Field `chrp_cnt_trig_src` writer - 16:16\\]
0x0: RSS_BOOKKEEPING_CHRP_CNT is incremented on every CHIRP_START 0x1: RSS_BOOKKEEPING_CHRP_CNT is incremented on every CHIRP_END"]
pub type ChrpCntTrigSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chrp_cnt_rst_src` reader - 20:20\\]
0x0: RSS_BOOKKEEPING_CHRP_CNT is reset on every FRAME_START 0x1: RSS_BOOKKEEPING_CHRP_CNT is reset by software"]
pub type ChrpCntRstSrcR = crate::BitReader;
#[doc = "Field `chrp_cnt_rst_src` writer - 20:20\\]
0x0: RSS_BOOKKEEPING_CHRP_CNT is reset on every FRAME_START 0x1: RSS_BOOKKEEPING_CHRP_CNT is reset by software"]
pub type ChrpCntRstSrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_SEQ_NUM"]
    #[inline(always)]
    pub fn seq_num_rst(&self) -> SeqNumRstR {
        SeqNumRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_FRM_CNT"]
    #[inline(always)]
    pub fn frm_cnt_rst(&self) -> FrmCntRstR {
        FrmCntRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_CHRP_CNT"]
    #[inline(always)]
    pub fn chrp_cnt_rst(&self) -> ChrpCntRstR {
        ChrpCntRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0x0: RSS_BOOKKEEPING_FRM_CNT is incremented on every FRAME_START 0x1: RSS_BOOKKEEPING_FRM_CNT is incremented on every FRAME_END"]
    #[inline(always)]
    pub fn frm_cnt_trig_src(&self) -> FrmCntTrigSrcR {
        FrmCntTrigSrcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0x0: RSS_BOOKKEEPING_CHRP_CNT is incremented on every CHIRP_START 0x1: RSS_BOOKKEEPING_CHRP_CNT is incremented on every CHIRP_END"]
    #[inline(always)]
    pub fn chrp_cnt_trig_src(&self) -> ChrpCntTrigSrcR {
        ChrpCntTrigSrcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0x0: RSS_BOOKKEEPING_CHRP_CNT is reset on every FRAME_START 0x1: RSS_BOOKKEEPING_CHRP_CNT is reset by software"]
    #[inline(always)]
    pub fn chrp_cnt_rst_src(&self) -> ChrpCntRstSrcR {
        ChrpCntRstSrcR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_SEQ_NUM"]
    #[inline(always)]
    #[must_use]
    pub fn seq_num_rst(&mut self) -> SeqNumRstW<RssBookkeepingCtrlSpec> {
        SeqNumRstW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_FRM_CNT"]
    #[inline(always)]
    #[must_use]
    pub fn frm_cnt_rst(&mut self) -> FrmCntRstW<RssBookkeepingCtrlSpec> {
        FrmCntRstW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 1'b1 resets the RSS_BOOKKEEPING_CHRP_CNT"]
    #[inline(always)]
    #[must_use]
    pub fn chrp_cnt_rst(&mut self) -> ChrpCntRstW<RssBookkeepingCtrlSpec> {
        ChrpCntRstW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
0x0: RSS_BOOKKEEPING_FRM_CNT is incremented on every FRAME_START 0x1: RSS_BOOKKEEPING_FRM_CNT is incremented on every FRAME_END"]
    #[inline(always)]
    #[must_use]
    pub fn frm_cnt_trig_src(&mut self) -> FrmCntTrigSrcW<RssBookkeepingCtrlSpec> {
        FrmCntTrigSrcW::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
0x0: RSS_BOOKKEEPING_CHRP_CNT is incremented on every CHIRP_START 0x1: RSS_BOOKKEEPING_CHRP_CNT is incremented on every CHIRP_END"]
    #[inline(always)]
    #[must_use]
    pub fn chrp_cnt_trig_src(&mut self) -> ChrpCntTrigSrcW<RssBookkeepingCtrlSpec> {
        ChrpCntTrigSrcW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
0x0: RSS_BOOKKEEPING_CHRP_CNT is reset on every FRAME_START 0x1: RSS_BOOKKEEPING_CHRP_CNT is reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn chrp_cnt_rst_src(&mut self) -> ChrpCntRstSrcW<RssBookkeepingCtrlSpec> {
        ChrpCntRstSrcW::new(self, 20)
    }
}
#[doc = "RSS_BOOKKEEPING_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bookkeeping_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bookkeeping_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssBookkeepingCtrlSpec;
impl crate::RegisterSpec for RssBookkeepingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_bookkeeping_ctrl::R`](R) reader structure"]
impl crate::Readable for RssBookkeepingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_bookkeeping_ctrl::W`](W) writer structure"]
impl crate::Writable for RssBookkeepingCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_BOOKKEEPING_CTRL to value 0"]
impl crate::Resettable for RssBookkeepingCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

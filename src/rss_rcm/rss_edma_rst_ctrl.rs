#[doc = "Register `RSS_EDMA_RST_CTRL` reader"]
pub type R = crate::R<RssEdmaRstCtrlSpec>;
#[doc = "Register `RSS_EDMA_RST_CTRL` writer"]
pub type W = crate::W<RssEdmaRstCtrlSpec>;
#[doc = "Field `assert` reader - 2:0\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS EDMA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertR = crate::FieldReader;
#[doc = "Field `assert` writer - 2:0\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS EDMA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tpcca_assert` reader - 6:4\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS EDMA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type TpccaAssertR = crate::FieldReader;
#[doc = "Field `tpcca_assert` writer - 6:4\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS EDMA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type TpccaAssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tptca0_assert` reader - 10:8\\]
writing '111' will reset MSS_TPCCA"]
pub type Tptca0AssertR = crate::FieldReader;
#[doc = "Field `tptca0_assert` writer - 10:8\\]
writing '111' will reset MSS_TPCCA"]
pub type Tptca0AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS EDMA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    pub fn assert(&self) -> AssertR {
        AssertR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS EDMA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    pub fn tpcca_assert(&self) -> TpccaAssertR {
        TpccaAssertR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
writing '111' will reset MSS_TPCCA"]
    #[inline(always)]
    pub fn tptca0_assert(&self) -> Tptca0AssertR {
        Tptca0AssertR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS EDMA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    #[must_use]
    pub fn assert(&mut self) -> AssertW<RssEdmaRstCtrlSpec> {
        AssertW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS EDMA Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    #[must_use]
    pub fn tpcca_assert(&mut self) -> TpccaAssertW<RssEdmaRstCtrlSpec> {
        TpccaAssertW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
writing '111' will reset MSS_TPCCA"]
    #[inline(always)]
    #[must_use]
    pub fn tptca0_assert(&mut self) -> Tptca0AssertW<RssEdmaRstCtrlSpec> {
        Tptca0AssertW::new(self, 8)
    }
}
#[doc = "RSS_EDMA_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_edma_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_edma_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssEdmaRstCtrlSpec;
impl crate::RegisterSpec for RssEdmaRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_edma_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for RssEdmaRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_edma_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for RssEdmaRstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_EDMA_RST_CTRL to value 0"]
impl crate::Resettable for RssEdmaRstCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

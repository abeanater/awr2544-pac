#[doc = "Register `RSS_BSS_RST_CTRL` reader"]
pub type R = crate::R<RssBssRstCtrlSpec>;
#[doc = "Register `RSS_BSS_RST_CTRL` writer"]
pub type W = crate::W<RssBssRstCtrlSpec>;
#[doc = "Field `assert` reader - 2:0\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS BSS Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertR = crate::FieldReader;
#[doc = "Field `assert` writer - 2:0\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS BSS Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
pub type AssertW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS BSS Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    pub fn assert(&self) -> AssertR {
        AssertR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This feature is for debug pupose only. software need to ensure the correct state of Device/IP before configuring this reset control for RSS BSS Data should be loaded as multibit. Write 3'b000: Reset is not asserted by SW. There could be another reset source which could reset the module. Refer to RCM spec for more details Write 3'b111 : Reset is asserted by SW"]
    #[inline(always)]
    #[must_use]
    pub fn assert(&mut self) -> AssertW<RssBssRstCtrlSpec> {
        AssertW::new(self, 0)
    }
}
#[doc = "RSS_BSS_RST_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_bss_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_bss_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssBssRstCtrlSpec;
impl crate::RegisterSpec for RssBssRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_bss_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for RssBssRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_bss_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for RssBssRstCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_BSS_RST_CTRL to value 0"]
impl crate::Resettable for RssBssRstCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

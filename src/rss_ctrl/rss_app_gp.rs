#[doc = "Register `RSS_APP_GP` reader"]
pub type R = crate::R<RssAppGpSpec>;
#[doc = "Register `RSS_APP_GP` writer"]
pub type W = crate::W<RssAppGpSpec>;
#[doc = "Field `reg` reader - 31:0\\]
General purpose register for application"]
pub type RegR = crate::FieldReader<u32>;
#[doc = "Field `reg` writer - 31:0\\]
General purpose register for application"]
pub type RegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
General purpose register for application"]
    #[inline(always)]
    pub fn reg(&self) -> RegR {
        RegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
General purpose register for application"]
    #[inline(always)]
    #[must_use]
    pub fn reg(&mut self) -> RegW<RssAppGpSpec> {
        RegW::new(self, 0)
    }
}
#[doc = "RSS_APP_GP\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_app_gp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_app_gp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssAppGpSpec;
impl crate::RegisterSpec for RssAppGpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_app_gp::R`](R) reader structure"]
impl crate::Readable for RssAppGpSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_app_gp::W`](W) writer structure"]
impl crate::Writable for RssAppGpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_APP_GP to value 0"]
impl crate::Resettable for RssAppGpSpec {
    const RESET_VALUE: u32 = 0;
}

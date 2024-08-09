#[doc = "Register `RSS_IP_ACCESS_DIS` reader"]
pub type R = crate::R<RssIpAccessDisSpec>;
#[doc = "Register `RSS_IP_ACCESS_DIS` writer"]
pub type W = crate::W<RssIpAccessDisSpec>;
#[doc = "Field `disable` reader - 31:0\\]
IP access disable configuration bits Writing 3'b111 will disable the IP"]
pub type DisableR = crate::FieldReader<u32>;
#[doc = "Field `disable` writer - 31:0\\]
IP access disable configuration bits Writing 3'b111 will disable the IP"]
pub type DisableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
IP access disable configuration bits Writing 3'b111 will disable the IP"]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
IP access disable configuration bits Writing 3'b111 will disable the IP"]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DisableW<RssIpAccessDisSpec> {
        DisableW::new(self, 0)
    }
}
#[doc = "RSS_IP_ACCESS_DIS\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_ip_access_dis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_ip_access_dis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssIpAccessDisSpec;
impl crate::RegisterSpec for RssIpAccessDisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_ip_access_dis::R`](R) reader structure"]
impl crate::Readable for RssIpAccessDisSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_ip_access_dis::W`](W) writer structure"]
impl crate::Writable for RssIpAccessDisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_IP_ACCESS_DIS to value 0"]
impl crate::Resettable for RssIpAccessDisSpec {
    const RESET_VALUE: u32 = 0;
}

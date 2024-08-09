#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_BOFFTEST_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacBofftestRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_BOFFTEST_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacBofftestRegSpec>;
#[doc = "Field `BACKOFF_COUNT` reader - 9:0\\]
Backoff Count"]
pub type BackoffCountR = crate::FieldReader<u16>;
#[doc = "Field `BACKOFF_COUNT` writer - 9:0\\]
Backoff Count"]
pub type BackoffCountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `COLLISION_COUNT` reader - 15:12\\]
Collision Count"]
pub type CollisionCountR = crate::FieldReader;
#[doc = "Field `COLLISION_COUNT` writer - 15:12\\]
Collision Count"]
pub type CollisionCountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BACKOFF_RANDOM_NUMBER` reader - 25:16\\]
Backoff Random Number Generator"]
pub type BackoffRandomNumberR = crate::FieldReader<u16>;
#[doc = "Field `BACKOFF_RANDOM_NUMBER` writer - 25:16\\]
Backoff Random Number Generator"]
pub type BackoffRandomNumberW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PACING_REGISTER_CURRENT` reader - 30:26\\]
Pacing Register Current Value"]
pub type PacingRegisterCurrentR = crate::FieldReader;
#[doc = "Field `PACING_REGISTER_CURRENT` writer - 30:26\\]
Pacing Register Current Value"]
pub type PacingRegisterCurrentW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Backoff Count"]
    #[inline(always)]
    pub fn backoff_count(&self) -> BackoffCountR {
        BackoffCountR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Collision Count"]
    #[inline(always)]
    pub fn collision_count(&self) -> CollisionCountR {
        CollisionCountR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Backoff Random Number Generator"]
    #[inline(always)]
    pub fn backoff_random_number(&self) -> BackoffRandomNumberR {
        BackoffRandomNumberR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Pacing Register Current Value"]
    #[inline(always)]
    pub fn pacing_register_current(&self) -> PacingRegisterCurrentR {
        PacingRegisterCurrentR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Backoff Count"]
    #[inline(always)]
    #[must_use]
    pub fn backoff_count(&mut self) -> BackoffCountW<CpswNcEthMac0PnMacBofftestRegSpec> {
        BackoffCountW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Collision Count"]
    #[inline(always)]
    #[must_use]
    pub fn collision_count(&mut self) -> CollisionCountW<CpswNcEthMac0PnMacBofftestRegSpec> {
        CollisionCountW::new(self, 12)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Backoff Random Number Generator"]
    #[inline(always)]
    #[must_use]
    pub fn backoff_random_number(
        &mut self,
    ) -> BackoffRandomNumberW<CpswNcEthMac0PnMacBofftestRegSpec> {
        BackoffRandomNumberW::new(self, 16)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Pacing Register Current Value"]
    #[inline(always)]
    #[must_use]
    pub fn pacing_register_current(
        &mut self,
    ) -> PacingRegisterCurrentW<CpswNcEthMac0PnMacBofftestRegSpec> {
        PacingRegisterCurrentW::new(self, 26)
    }
}
#[doc = "Enet Port N Mac Backoff Test\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_bofftest_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_bofftest_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacBofftestRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacBofftestRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_bofftest_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacBofftestRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_bofftest_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacBofftestRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_BOFFTEST_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacBofftestRegSpec {
    const RESET_VALUE: u32 = 0;
}

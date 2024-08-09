#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_RX_PRI_MAP_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnRxPriMapRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_RX_PRI_MAP_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnRxPriMapRegSpec>;
#[doc = "Field `PRIORITY_0` reader - 2:0\\]
Priority 0"]
pub type Priority0R = crate::FieldReader;
#[doc = "Field `PRIORITY_0` writer - 2:0\\]
Priority 0"]
pub type Priority0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIORITY_1` reader - 6:4\\]
Priority 1"]
pub type Priority1R = crate::FieldReader;
#[doc = "Field `PRIORITY_1` writer - 6:4\\]
Priority 1"]
pub type Priority1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIORITY_2` reader - 10:8\\]
Priority 2"]
pub type Priority2R = crate::FieldReader;
#[doc = "Field `PRIORITY_2` writer - 10:8\\]
Priority 2"]
pub type Priority2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIORITY_3` reader - 14:12\\]
Priority 3"]
pub type Priority3R = crate::FieldReader;
#[doc = "Field `PRIORITY_3` writer - 14:12\\]
Priority 3"]
pub type Priority3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIORITY_4` reader - 18:16\\]
Priority 4"]
pub type Priority4R = crate::FieldReader;
#[doc = "Field `PRIORITY_4` writer - 18:16\\]
Priority 4"]
pub type Priority4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIORITY_5` reader - 22:20\\]
Priority 5"]
pub type Priority5R = crate::FieldReader;
#[doc = "Field `PRIORITY_5` writer - 22:20\\]
Priority 5"]
pub type Priority5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIORITY_6` reader - 26:24\\]
Priority 6"]
pub type Priority6R = crate::FieldReader;
#[doc = "Field `PRIORITY_6` writer - 26:24\\]
Priority 6"]
pub type Priority6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIORITY_7` reader - 30:28\\]
Priority 7"]
pub type Priority7R = crate::FieldReader;
#[doc = "Field `PRIORITY_7` writer - 30:28\\]
Priority 7"]
pub type Priority7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Priority 0"]
    #[inline(always)]
    pub fn priority_0(&self) -> Priority0R {
        Priority0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Priority 1"]
    #[inline(always)]
    pub fn priority_1(&self) -> Priority1R {
        Priority1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Priority 2"]
    #[inline(always)]
    pub fn priority_2(&self) -> Priority2R {
        Priority2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Priority 3"]
    #[inline(always)]
    pub fn priority_3(&self) -> Priority3R {
        Priority3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Priority 4"]
    #[inline(always)]
    pub fn priority_4(&self) -> Priority4R {
        Priority4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Priority 5"]
    #[inline(always)]
    pub fn priority_5(&self) -> Priority5R {
        Priority5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Priority 6"]
    #[inline(always)]
    pub fn priority_6(&self) -> Priority6R {
        Priority6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Priority 7"]
    #[inline(always)]
    pub fn priority_7(&self) -> Priority7R {
        Priority7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Priority 0"]
    #[inline(always)]
    #[must_use]
    pub fn priority_0(&mut self) -> Priority0W<CpswNcEthMac0PnRxPriMapRegSpec> {
        Priority0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Priority 1"]
    #[inline(always)]
    #[must_use]
    pub fn priority_1(&mut self) -> Priority1W<CpswNcEthMac0PnRxPriMapRegSpec> {
        Priority1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Priority 2"]
    #[inline(always)]
    #[must_use]
    pub fn priority_2(&mut self) -> Priority2W<CpswNcEthMac0PnRxPriMapRegSpec> {
        Priority2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Priority 3"]
    #[inline(always)]
    #[must_use]
    pub fn priority_3(&mut self) -> Priority3W<CpswNcEthMac0PnRxPriMapRegSpec> {
        Priority3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Priority 4"]
    #[inline(always)]
    #[must_use]
    pub fn priority_4(&mut self) -> Priority4W<CpswNcEthMac0PnRxPriMapRegSpec> {
        Priority4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Priority 5"]
    #[inline(always)]
    #[must_use]
    pub fn priority_5(&mut self) -> Priority5W<CpswNcEthMac0PnRxPriMapRegSpec> {
        Priority5W::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Priority 6"]
    #[inline(always)]
    #[must_use]
    pub fn priority_6(&mut self) -> Priority6W<CpswNcEthMac0PnRxPriMapRegSpec> {
        Priority6W::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Priority 7"]
    #[inline(always)]
    #[must_use]
    pub fn priority_7(&mut self) -> Priority7W<CpswNcEthMac0PnRxPriMapRegSpec> {
        Priority7W::new(self, 28)
    }
}
#[doc = "Enet Port N RX Pkt Pri to Header Pri Map\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_rx_pri_map_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_rx_pri_map_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnRxPriMapRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnRxPriMapRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_rx_pri_map_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnRxPriMapRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_rx_pri_map_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnRxPriMapRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_RX_PRI_MAP_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnRxPriMapRegSpec {
    const RESET_VALUE: u32 = 0;
}

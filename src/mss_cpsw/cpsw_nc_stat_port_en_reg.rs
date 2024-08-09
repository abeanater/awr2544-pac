#[doc = "Register `CPSW_NC_STAT_PORT_EN_REG` reader"]
pub type R = crate::R<CpswNcStatPortEnRegSpec>;
#[doc = "Register `CPSW_NC_STAT_PORT_EN_REG` writer"]
pub type W = crate::W<CpswNcStatPortEnRegSpec>;
#[doc = "Field `PORT_0_STATISTICS` reader - 0:0\\]
Port 0 Statistics Enable"]
pub type Port0StatisticsR = crate::BitReader;
#[doc = "Field `PORT_0_STATISTICS` writer - 0:0\\]
Port 0 Statistics Enable"]
pub type Port0StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_1_STATISTICS` reader - 1:1\\]
Port 1 Statistics Enable"]
pub type Port1StatisticsR = crate::BitReader;
#[doc = "Field `PORT_1_STATISTICS` writer - 1:1\\]
Port 1 Statistics Enable"]
pub type Port1StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_2_STATISTICS` reader - 2:2\\]
Port 2 Statistics Enable"]
pub type Port2StatisticsR = crate::BitReader;
#[doc = "Field `PORT_2_STATISTICS` writer - 2:2\\]
Port 2 Statistics Enable"]
pub type Port2StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_3_STATISTICS` reader - 3:3\\]
Port 3 Statistics Enable"]
pub type Port3StatisticsR = crate::BitReader;
#[doc = "Field `PORT_3_STATISTICS` writer - 3:3\\]
Port 3 Statistics Enable"]
pub type Port3StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_4_STATISTICS` reader - 4:4\\]
Port 4 Statistics Enable"]
pub type Port4StatisticsR = crate::BitReader;
#[doc = "Field `PORT_4_STATISTICS` writer - 4:4\\]
Port 4 Statistics Enable"]
pub type Port4StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_5_STATISTICS` reader - 5:5\\]
Port 5 Statistics Enable"]
pub type Port5StatisticsR = crate::BitReader;
#[doc = "Field `PORT_5_STATISTICS` writer - 5:5\\]
Port 5 Statistics Enable"]
pub type Port5StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_6_STATISTICS` reader - 6:6\\]
Port 6 Statistics Enable"]
pub type Port6StatisticsR = crate::BitReader;
#[doc = "Field `PORT_6_STATISTICS` writer - 6:6\\]
Port 6 Statistics Enable"]
pub type Port6StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_7_STATISTICS` reader - 7:7\\]
Port 7 Statistics Enable"]
pub type Port7StatisticsR = crate::BitReader;
#[doc = "Field `PORT_7_STATISTICS` writer - 7:7\\]
Port 7 Statistics Enable"]
pub type Port7StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORT_8_STATISTICS` reader - 8:8\\]
Port 8 Statistics Enable"]
pub type Port8StatisticsR = crate::BitReader;
#[doc = "Field `PORT_8_STATISTICS` writer - 8:8\\]
Port 8 Statistics Enable"]
pub type Port8StatisticsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Port 0 Statistics Enable"]
    #[inline(always)]
    pub fn port_0_statistics(&self) -> Port0StatisticsR {
        Port0StatisticsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Port 1 Statistics Enable"]
    #[inline(always)]
    pub fn port_1_statistics(&self) -> Port1StatisticsR {
        Port1StatisticsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 2 Statistics Enable"]
    #[inline(always)]
    pub fn port_2_statistics(&self) -> Port2StatisticsR {
        Port2StatisticsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 3 Statistics Enable"]
    #[inline(always)]
    pub fn port_3_statistics(&self) -> Port3StatisticsR {
        Port3StatisticsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Port 4 Statistics Enable"]
    #[inline(always)]
    pub fn port_4_statistics(&self) -> Port4StatisticsR {
        Port4StatisticsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Port 5 Statistics Enable"]
    #[inline(always)]
    pub fn port_5_statistics(&self) -> Port5StatisticsR {
        Port5StatisticsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Port 6 Statistics Enable"]
    #[inline(always)]
    pub fn port_6_statistics(&self) -> Port6StatisticsR {
        Port6StatisticsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Port 7 Statistics Enable"]
    #[inline(always)]
    pub fn port_7_statistics(&self) -> Port7StatisticsR {
        Port7StatisticsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 8 Statistics Enable"]
    #[inline(always)]
    pub fn port_8_statistics(&self) -> Port8StatisticsR {
        Port8StatisticsR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Port 0 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_0_statistics(&mut self) -> Port0StatisticsW<CpswNcStatPortEnRegSpec> {
        Port0StatisticsW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Port 1 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_1_statistics(&mut self) -> Port1StatisticsW<CpswNcStatPortEnRegSpec> {
        Port1StatisticsW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Port 2 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_2_statistics(&mut self) -> Port2StatisticsW<CpswNcStatPortEnRegSpec> {
        Port2StatisticsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Port 3 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_3_statistics(&mut self) -> Port3StatisticsW<CpswNcStatPortEnRegSpec> {
        Port3StatisticsW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Port 4 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_4_statistics(&mut self) -> Port4StatisticsW<CpswNcStatPortEnRegSpec> {
        Port4StatisticsW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Port 5 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_5_statistics(&mut self) -> Port5StatisticsW<CpswNcStatPortEnRegSpec> {
        Port5StatisticsW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Port 6 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_6_statistics(&mut self) -> Port6StatisticsW<CpswNcStatPortEnRegSpec> {
        Port6StatisticsW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Port 7 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_7_statistics(&mut self) -> Port7StatisticsW<CpswNcStatPortEnRegSpec> {
        Port7StatisticsW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Port 8 Statistics Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_8_statistics(&mut self) -> Port8StatisticsW<CpswNcStatPortEnRegSpec> {
        Port8StatisticsW::new(self, 8)
    }
}
#[doc = "CPSW Statistics Port Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_port_en_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_port_en_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStatPortEnRegSpec;
impl crate::RegisterSpec for CpswNcStatPortEnRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_port_en_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcStatPortEnRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_port_en_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcStatPortEnRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_PORT_EN_REG to value 0"]
impl crate::Resettable for CpswNcStatPortEnRegSpec {
    const RESET_VALUE: u32 = 0;
}

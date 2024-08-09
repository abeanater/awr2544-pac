#[doc = "Register `CPSW_NC_CPPI_P0_EEE_STATUS_REG` reader"]
pub type R = crate::R<CpswNcCppiP0EeeStatusRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_EEE_STATUS_REG` writer"]
pub type W = crate::W<CpswNcCppiP0EeeStatusRegSpec>;
#[doc = "Field `CPPI_PORT_0_6` reader - 0:0\\]
CPPI port 0 wait idle to LPI - asserted when port 0 is counting the IDLE2LPI time"]
pub type CppiPort0_6R = crate::BitReader;
#[doc = "Field `CPPI_PORT_0_6` writer - 0:0\\]
CPPI port 0 wait idle to LPI - asserted when port 0 is counting the IDLE2LPI time"]
pub type CppiPort0_6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPPI_PORT_0_5` reader - 1:1\\]
CPPI port 0 receive LPI state - asserted when the port 0 receive is in the LPI state"]
pub type CppiPort0_5R = crate::BitReader;
#[doc = "Field `CPPI_PORT_0_5` writer - 1:1\\]
CPPI port 0 receive LPI state - asserted when the port 0 receive is in the LPI state"]
pub type CppiPort0_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPPI_PORT_0_4` reader - 2:2\\]
CPPI port 0 transmit LPI state - asserted when the port 0 transmit is in the LPI state"]
pub type CppiPort0_4R = crate::BitReader;
#[doc = "Field `CPPI_PORT_0_4` writer - 2:2\\]
CPPI port 0 transmit LPI state - asserted when the port 0 transmit is in the LPI state"]
pub type CppiPort0_4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPPI_PORT_0_3` reader - 3:3\\]
CPPI port 0 transmit wakeup - asserted in the transmit LPI2WAKE count time"]
pub type CppiPort0_3R = crate::BitReader;
#[doc = "Field `CPPI_PORT_0_3` writer - 3:3\\]
CPPI port 0 transmit wakeup - asserted in the transmit LPI2WAKE count time"]
pub type CppiPort0_3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPPI_PORT_0_2` reader - 4:4\\]
CPPI port 0 transmit FIFO hold - asserted in the LPI state and during the LPI2WAKE count time"]
pub type CppiPort0_2R = crate::BitReader;
#[doc = "Field `CPPI_PORT_0_2` writer - 4:4\\]
CPPI port 0 transmit FIFO hold - asserted in the LPI state and during the LPI2WAKE count time"]
pub type CppiPort0_2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPPI_PORT_0_1` reader - 5:5\\]
CPPI port 0 receive FIFO (switch ingress) is empty - contains no packets"]
pub type CppiPort0_1R = crate::BitReader;
#[doc = "Field `CPPI_PORT_0_1` writer - 5:5\\]
CPPI port 0 receive FIFO (switch ingress) is empty - contains no packets"]
pub type CppiPort0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPPI_PORT_0` reader - 6:6\\]
CPPI port 0 transmit FIFO (switch egress) is empty - contains no packets"]
pub type CppiPort0R = crate::BitReader;
#[doc = "Field `CPPI_PORT_0` writer - 6:6\\]
CPPI port 0 transmit FIFO (switch egress) is empty - contains no packets"]
pub type CppiPort0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CPPI port 0 wait idle to LPI - asserted when port 0 is counting the IDLE2LPI time"]
    #[inline(always)]
    pub fn cppi_port_0_6(&self) -> CppiPort0_6R {
        CppiPort0_6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPPI port 0 receive LPI state - asserted when the port 0 receive is in the LPI state"]
    #[inline(always)]
    pub fn cppi_port_0_5(&self) -> CppiPort0_5R {
        CppiPort0_5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CPPI port 0 transmit LPI state - asserted when the port 0 transmit is in the LPI state"]
    #[inline(always)]
    pub fn cppi_port_0_4(&self) -> CppiPort0_4R {
        CppiPort0_4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
CPPI port 0 transmit wakeup - asserted in the transmit LPI2WAKE count time"]
    #[inline(always)]
    pub fn cppi_port_0_3(&self) -> CppiPort0_3R {
        CppiPort0_3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
CPPI port 0 transmit FIFO hold - asserted in the LPI state and during the LPI2WAKE count time"]
    #[inline(always)]
    pub fn cppi_port_0_2(&self) -> CppiPort0_2R {
        CppiPort0_2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
CPPI port 0 receive FIFO (switch ingress) is empty - contains no packets"]
    #[inline(always)]
    pub fn cppi_port_0_1(&self) -> CppiPort0_1R {
        CppiPort0_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
CPPI port 0 transmit FIFO (switch egress) is empty - contains no packets"]
    #[inline(always)]
    pub fn cppi_port_0(&self) -> CppiPort0R {
        CppiPort0R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CPPI port 0 wait idle to LPI - asserted when port 0 is counting the IDLE2LPI time"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_port_0_6(&mut self) -> CppiPort0_6W<CpswNcCppiP0EeeStatusRegSpec> {
        CppiPort0_6W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CPPI port 0 receive LPI state - asserted when the port 0 receive is in the LPI state"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_port_0_5(&mut self) -> CppiPort0_5W<CpswNcCppiP0EeeStatusRegSpec> {
        CppiPort0_5W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CPPI port 0 transmit LPI state - asserted when the port 0 transmit is in the LPI state"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_port_0_4(&mut self) -> CppiPort0_4W<CpswNcCppiP0EeeStatusRegSpec> {
        CppiPort0_4W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
CPPI port 0 transmit wakeup - asserted in the transmit LPI2WAKE count time"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_port_0_3(&mut self) -> CppiPort0_3W<CpswNcCppiP0EeeStatusRegSpec> {
        CppiPort0_3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
CPPI port 0 transmit FIFO hold - asserted in the LPI state and during the LPI2WAKE count time"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_port_0_2(&mut self) -> CppiPort0_2W<CpswNcCppiP0EeeStatusRegSpec> {
        CppiPort0_2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
CPPI port 0 receive FIFO (switch ingress) is empty - contains no packets"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_port_0_1(&mut self) -> CppiPort0_1W<CpswNcCppiP0EeeStatusRegSpec> {
        CppiPort0_1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
CPPI port 0 transmit FIFO (switch egress) is empty - contains no packets"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_port_0(&mut self) -> CppiPort0W<CpswNcCppiP0EeeStatusRegSpec> {
        CppiPort0W::new(self, 6)
    }
}
#[doc = "Port 0 EEE status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_eee_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_eee_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0EeeStatusRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0EeeStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_eee_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0EeeStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_eee_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0EeeStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_EEE_STATUS_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0EeeStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}

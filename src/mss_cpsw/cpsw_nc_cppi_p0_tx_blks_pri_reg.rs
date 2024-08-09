#[doc = "Register `CPSW_NC_CPPI_P0_TX_BLKS_PRI_REG` reader"]
pub type R = crate::R<CpswNcCppiP0TxBlksPriRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_TX_BLKS_PRI_REG` writer"]
pub type W = crate::W<CpswNcCppiP0TxBlksPriRegSpec>;
#[doc = "Field `PRIORITY_0_PORT` reader - 3:0\\]
Priority 0 Port Transmit Blocks"]
pub type Priority0PortR = crate::FieldReader;
#[doc = "Field `PRIORITY_0_PORT` writer - 3:0\\]
Priority 0 Port Transmit Blocks"]
pub type Priority0PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_1_PORT` reader - 7:4\\]
Priority 1 Port Transmit Blocks"]
pub type Priority1PortR = crate::FieldReader;
#[doc = "Field `PRIORITY_1_PORT` writer - 7:4\\]
Priority 1 Port Transmit Blocks"]
pub type Priority1PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_2_PORT` reader - 11:8\\]
Priority 2 Port Transmit Blocks"]
pub type Priority2PortR = crate::FieldReader;
#[doc = "Field `PRIORITY_2_PORT` writer - 11:8\\]
Priority 2 Port Transmit Blocks"]
pub type Priority2PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_3_PORT` reader - 15:12\\]
Priority 3 Port Transmit Blocks"]
pub type Priority3PortR = crate::FieldReader;
#[doc = "Field `PRIORITY_3_PORT` writer - 15:12\\]
Priority 3 Port Transmit Blocks"]
pub type Priority3PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_4_PORT` reader - 19:16\\]
Priority 4 Port Transmit Blocks"]
pub type Priority4PortR = crate::FieldReader;
#[doc = "Field `PRIORITY_4_PORT` writer - 19:16\\]
Priority 4 Port Transmit Blocks"]
pub type Priority4PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_5_PORT` reader - 23:20\\]
Priority 5 Port Transmit Blocks"]
pub type Priority5PortR = crate::FieldReader;
#[doc = "Field `PRIORITY_5_PORT` writer - 23:20\\]
Priority 5 Port Transmit Blocks"]
pub type Priority5PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_6_PORT` reader - 27:24\\]
Priority 6 Port Transmit Blocks"]
pub type Priority6PortR = crate::FieldReader;
#[doc = "Field `PRIORITY_6_PORT` writer - 27:24\\]
Priority 6 Port Transmit Blocks"]
pub type Priority6PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_7_PORT` reader - 31:28\\]
Priority 7 Port Transmit Blocks"]
pub type Priority7PortR = crate::FieldReader;
#[doc = "Field `PRIORITY_7_PORT` writer - 31:28\\]
Priority 7 Port Transmit Blocks"]
pub type Priority7PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Priority 0 Port Transmit Blocks"]
    #[inline(always)]
    pub fn priority_0_port(&self) -> Priority0PortR {
        Priority0PortR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Priority 1 Port Transmit Blocks"]
    #[inline(always)]
    pub fn priority_1_port(&self) -> Priority1PortR {
        Priority1PortR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Priority 2 Port Transmit Blocks"]
    #[inline(always)]
    pub fn priority_2_port(&self) -> Priority2PortR {
        Priority2PortR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Priority 3 Port Transmit Blocks"]
    #[inline(always)]
    pub fn priority_3_port(&self) -> Priority3PortR {
        Priority3PortR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority 4 Port Transmit Blocks"]
    #[inline(always)]
    pub fn priority_4_port(&self) -> Priority4PortR {
        Priority4PortR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Priority 5 Port Transmit Blocks"]
    #[inline(always)]
    pub fn priority_5_port(&self) -> Priority5PortR {
        Priority5PortR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Priority 6 Port Transmit Blocks"]
    #[inline(always)]
    pub fn priority_6_port(&self) -> Priority6PortR {
        Priority6PortR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Priority 7 Port Transmit Blocks"]
    #[inline(always)]
    pub fn priority_7_port(&self) -> Priority7PortR {
        Priority7PortR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Priority 0 Port Transmit Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_0_port(&mut self) -> Priority0PortW<CpswNcCppiP0TxBlksPriRegSpec> {
        Priority0PortW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Priority 1 Port Transmit Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_1_port(&mut self) -> Priority1PortW<CpswNcCppiP0TxBlksPriRegSpec> {
        Priority1PortW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Priority 2 Port Transmit Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_2_port(&mut self) -> Priority2PortW<CpswNcCppiP0TxBlksPriRegSpec> {
        Priority2PortW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Priority 3 Port Transmit Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_3_port(&mut self) -> Priority3PortW<CpswNcCppiP0TxBlksPriRegSpec> {
        Priority3PortW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority 4 Port Transmit Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_4_port(&mut self) -> Priority4PortW<CpswNcCppiP0TxBlksPriRegSpec> {
        Priority4PortW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Priority 5 Port Transmit Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_5_port(&mut self) -> Priority5PortW<CpswNcCppiP0TxBlksPriRegSpec> {
        Priority5PortW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Priority 6 Port Transmit Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_6_port(&mut self) -> Priority6PortW<CpswNcCppiP0TxBlksPriRegSpec> {
        Priority6PortW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Priority 7 Port Transmit Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_7_port(&mut self) -> Priority7PortW<CpswNcCppiP0TxBlksPriRegSpec> {
        Priority7PortW::new(self, 28)
    }
}
#[doc = "CPPI Port 0 Transmit Block Sub Per Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_tx_blks_pri_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_tx_blks_pri_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0TxBlksPriRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0TxBlksPriRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_tx_blks_pri_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0TxBlksPriRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_tx_blks_pri_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0TxBlksPriRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_TX_BLKS_PRI_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0TxBlksPriRegSpec {
    const RESET_VALUE: u32 = 0;
}

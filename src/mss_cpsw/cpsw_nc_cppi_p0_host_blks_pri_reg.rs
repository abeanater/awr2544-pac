#[doc = "Register `CPSW_NC_CPPI_P0_HOST_BLKS_PRI_REG` reader"]
pub type R = crate::R<CpswNcCppiP0HostBlksPriRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_HOST_BLKS_PRI_REG` writer"]
pub type W = crate::W<CpswNcCppiP0HostBlksPriRegSpec>;
#[doc = "Field `PRIORITY_0_HOST` reader - 3:0\\]
Priority 0 Host Blocks"]
pub type Priority0HostR = crate::FieldReader;
#[doc = "Field `PRIORITY_0_HOST` writer - 3:0\\]
Priority 0 Host Blocks"]
pub type Priority0HostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_1_HOST` reader - 7:4\\]
Priority 1 Host Blocks"]
pub type Priority1HostR = crate::FieldReader;
#[doc = "Field `PRIORITY_1_HOST` writer - 7:4\\]
Priority 1 Host Blocks"]
pub type Priority1HostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_2_HOST` reader - 11:8\\]
Priority 2 Host Blocks"]
pub type Priority2HostR = crate::FieldReader;
#[doc = "Field `PRIORITY_2_HOST` writer - 11:8\\]
Priority 2 Host Blocks"]
pub type Priority2HostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_3_HOST` reader - 15:12\\]
Priority 3 Host Blocks"]
pub type Priority3HostR = crate::FieldReader;
#[doc = "Field `PRIORITY_3_HOST` writer - 15:12\\]
Priority 3 Host Blocks"]
pub type Priority3HostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_4_HOST` reader - 19:16\\]
Priority 4 Host Blocks"]
pub type Priority4HostR = crate::FieldReader;
#[doc = "Field `PRIORITY_4_HOST` writer - 19:16\\]
Priority 4 Host Blocks"]
pub type Priority4HostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_5_HOST` reader - 23:20\\]
Priority 5 Host Blocks"]
pub type Priority5HostR = crate::FieldReader;
#[doc = "Field `PRIORITY_5_HOST` writer - 23:20\\]
Priority 5 Host Blocks"]
pub type Priority5HostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_6_HOST` reader - 27:24\\]
Priority 6 Host Blocks"]
pub type Priority6HostR = crate::FieldReader;
#[doc = "Field `PRIORITY_6_HOST` writer - 27:24\\]
Priority 6 Host Blocks"]
pub type Priority6HostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIORITY_7_HOST` reader - 31:28\\]
Priority 7 Host Blocks"]
pub type Priority7HostR = crate::FieldReader;
#[doc = "Field `PRIORITY_7_HOST` writer - 31:28\\]
Priority 7 Host Blocks"]
pub type Priority7HostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Priority 0 Host Blocks"]
    #[inline(always)]
    pub fn priority_0_host(&self) -> Priority0HostR {
        Priority0HostR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Priority 1 Host Blocks"]
    #[inline(always)]
    pub fn priority_1_host(&self) -> Priority1HostR {
        Priority1HostR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Priority 2 Host Blocks"]
    #[inline(always)]
    pub fn priority_2_host(&self) -> Priority2HostR {
        Priority2HostR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Priority 3 Host Blocks"]
    #[inline(always)]
    pub fn priority_3_host(&self) -> Priority3HostR {
        Priority3HostR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority 4 Host Blocks"]
    #[inline(always)]
    pub fn priority_4_host(&self) -> Priority4HostR {
        Priority4HostR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Priority 5 Host Blocks"]
    #[inline(always)]
    pub fn priority_5_host(&self) -> Priority5HostR {
        Priority5HostR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Priority 6 Host Blocks"]
    #[inline(always)]
    pub fn priority_6_host(&self) -> Priority6HostR {
        Priority6HostR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Priority 7 Host Blocks"]
    #[inline(always)]
    pub fn priority_7_host(&self) -> Priority7HostR {
        Priority7HostR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Priority 0 Host Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_0_host(&mut self) -> Priority0HostW<CpswNcCppiP0HostBlksPriRegSpec> {
        Priority0HostW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Priority 1 Host Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_1_host(&mut self) -> Priority1HostW<CpswNcCppiP0HostBlksPriRegSpec> {
        Priority1HostW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Priority 2 Host Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_2_host(&mut self) -> Priority2HostW<CpswNcCppiP0HostBlksPriRegSpec> {
        Priority2HostW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Priority 3 Host Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_3_host(&mut self) -> Priority3HostW<CpswNcCppiP0HostBlksPriRegSpec> {
        Priority3HostW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority 4 Host Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_4_host(&mut self) -> Priority4HostW<CpswNcCppiP0HostBlksPriRegSpec> {
        Priority4HostW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Priority 5 Host Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_5_host(&mut self) -> Priority5HostW<CpswNcCppiP0HostBlksPriRegSpec> {
        Priority5HostW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Priority 6 Host Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_6_host(&mut self) -> Priority6HostW<CpswNcCppiP0HostBlksPriRegSpec> {
        Priority6HostW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Priority 7 Host Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn priority_7_host(&mut self) -> Priority7HostW<CpswNcCppiP0HostBlksPriRegSpec> {
        Priority7HostW::new(self, 28)
    }
}
#[doc = "CPPI Port 0 Host Blocks Priority\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_host_blks_pri_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_host_blks_pri_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0HostBlksPriRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0HostBlksPriRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_host_blks_pri_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0HostBlksPriRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_host_blks_pri_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0HostBlksPriRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_HOST_BLKS_PRI_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0HostBlksPriRegSpec {
    const RESET_VALUE: u32 = 0;
}

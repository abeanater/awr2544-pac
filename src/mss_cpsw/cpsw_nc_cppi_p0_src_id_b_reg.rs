#[doc = "Register `CPSW_NC_CPPI_P0_SRC_ID_B_REG` reader"]
pub type R = crate::R<CpswNcCppiP0SrcIdBRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_SRC_ID_B_REG` writer"]
pub type W = crate::W<CpswNcCppiP0SrcIdBRegSpec>;
#[doc = "Field `PORT_5_CPPI` reader - 7:0\\]
Port 5 CPPI Info Word0 Source ID Value"]
pub type Port5CppiR = crate::FieldReader;
#[doc = "Field `PORT_5_CPPI` writer - 7:0\\]
Port 5 CPPI Info Word0 Source ID Value"]
pub type Port5CppiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PORT_6_CPPI` reader - 15:8\\]
Port 6 CPPI Info Word0 Source ID Value"]
pub type Port6CppiR = crate::FieldReader;
#[doc = "Field `PORT_6_CPPI` writer - 15:8\\]
Port 6 CPPI Info Word0 Source ID Value"]
pub type Port6CppiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PORT_7_CPPI` reader - 23:16\\]
Port 7 CPPI Info Word0 Source ID Value"]
pub type Port7CppiR = crate::FieldReader;
#[doc = "Field `PORT_7_CPPI` writer - 23:16\\]
Port 7 CPPI Info Word0 Source ID Value"]
pub type Port7CppiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PORT_8_CPPI` reader - 31:24\\]
Port 8 CPPI Info Word0 Source ID Value"]
pub type Port8CppiR = crate::FieldReader;
#[doc = "Field `PORT_8_CPPI` writer - 31:24\\]
Port 8 CPPI Info Word0 Source ID Value"]
pub type Port8CppiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Port 5 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    pub fn port_5_cppi(&self) -> Port5CppiR {
        Port5CppiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Port 6 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    pub fn port_6_cppi(&self) -> Port6CppiR {
        Port6CppiR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Port 7 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    pub fn port_7_cppi(&self) -> Port7CppiR {
        Port7CppiR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Port 8 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    pub fn port_8_cppi(&self) -> Port8CppiR {
        Port8CppiR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Port 5 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    #[must_use]
    pub fn port_5_cppi(&mut self) -> Port5CppiW<CpswNcCppiP0SrcIdBRegSpec> {
        Port5CppiW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Port 6 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    #[must_use]
    pub fn port_6_cppi(&mut self) -> Port6CppiW<CpswNcCppiP0SrcIdBRegSpec> {
        Port6CppiW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Port 7 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    #[must_use]
    pub fn port_7_cppi(&mut self) -> Port7CppiW<CpswNcCppiP0SrcIdBRegSpec> {
        Port7CppiW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Port 8 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    #[must_use]
    pub fn port_8_cppi(&mut self) -> Port8CppiW<CpswNcCppiP0SrcIdBRegSpec> {
        Port8CppiW::new(self, 24)
    }
}
#[doc = "CPPI Port 0 CPPI Source ID B\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_src_id_b_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_src_id_b_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0SrcIdBRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0SrcIdBRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_src_id_b_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0SrcIdBRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_src_id_b_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0SrcIdBRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_SRC_ID_B_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0SrcIdBRegSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CPSW_NC_CPPI_P0_SRC_ID_A_REG` reader"]
pub type R = crate::R<CpswNcCppiP0SrcIdARegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_SRC_ID_A_REG` writer"]
pub type W = crate::W<CpswNcCppiP0SrcIdARegSpec>;
#[doc = "Field `PORT_1_CPPI` reader - 7:0\\]
Port 1 CPPI Info Word0 Source ID Value"]
pub type Port1CppiR = crate::FieldReader;
#[doc = "Field `PORT_1_CPPI` writer - 7:0\\]
Port 1 CPPI Info Word0 Source ID Value"]
pub type Port1CppiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PORT_2_CPPI` reader - 15:8\\]
Port 2 CPPI Info Word0 Source ID Value"]
pub type Port2CppiR = crate::FieldReader;
#[doc = "Field `PORT_2_CPPI` writer - 15:8\\]
Port 2 CPPI Info Word0 Source ID Value"]
pub type Port2CppiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PORT_3_CPPI` reader - 23:16\\]
Port 3 CPPI Info Word0 Source ID Value"]
pub type Port3CppiR = crate::FieldReader;
#[doc = "Field `PORT_3_CPPI` writer - 23:16\\]
Port 3 CPPI Info Word0 Source ID Value"]
pub type Port3CppiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PORT_4_CPPI` reader - 31:24\\]
Port 4 CPPI Info Word0 Source ID Value"]
pub type Port4CppiR = crate::FieldReader;
#[doc = "Field `PORT_4_CPPI` writer - 31:24\\]
Port 4 CPPI Info Word0 Source ID Value"]
pub type Port4CppiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Port 1 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    pub fn port_1_cppi(&self) -> Port1CppiR {
        Port1CppiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Port 2 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    pub fn port_2_cppi(&self) -> Port2CppiR {
        Port2CppiR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Port 3 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    pub fn port_3_cppi(&self) -> Port3CppiR {
        Port3CppiR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Port 4 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    pub fn port_4_cppi(&self) -> Port4CppiR {
        Port4CppiR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Port 1 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    #[must_use]
    pub fn port_1_cppi(&mut self) -> Port1CppiW<CpswNcCppiP0SrcIdARegSpec> {
        Port1CppiW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Port 2 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    #[must_use]
    pub fn port_2_cppi(&mut self) -> Port2CppiW<CpswNcCppiP0SrcIdARegSpec> {
        Port2CppiW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Port 3 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    #[must_use]
    pub fn port_3_cppi(&mut self) -> Port3CppiW<CpswNcCppiP0SrcIdARegSpec> {
        Port3CppiW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Port 4 CPPI Info Word0 Source ID Value"]
    #[inline(always)]
    #[must_use]
    pub fn port_4_cppi(&mut self) -> Port4CppiW<CpswNcCppiP0SrcIdARegSpec> {
        Port4CppiW::new(self, 24)
    }
}
#[doc = "CPPI Port 0 CPPI Source ID A\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_src_id_a_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_src_id_a_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0SrcIdARegSpec;
impl crate::RegisterSpec for CpswNcCppiP0SrcIdARegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_src_id_a_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0SrcIdARegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_src_id_a_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0SrcIdARegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_SRC_ID_A_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0SrcIdARegSpec {
    const RESET_VALUE: u32 = 0;
}

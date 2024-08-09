#[doc = "Register `DSS_TPCC_MEMINIT_START` reader"]
pub type R = crate::R<DssTpccMeminitStartSpec>;
#[doc = "Register `DSS_TPCC_MEMINIT_START` writer"]
pub type W = crate::W<DssTpccMeminitStartSpec>;
#[doc = "Field `tpcc_a_meminit_start` reader - 0:0\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie TPCC_A_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear TPCC_A_MEMINIT_DONE)"]
pub type TpccAMeminitStartR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_start` writer - 0:0\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie TPCC_A_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear TPCC_A_MEMINIT_DONE)"]
pub type TpccAMeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_meminit_start` reader - 1:1\\]
RESERVED: Dont Use"]
pub type TpccBMeminitStartR = crate::BitReader;
#[doc = "Field `tpcc_b_meminit_start` writer - 1:1\\]
RESERVED: Dont Use"]
pub type TpccBMeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_c_meminit_start` reader - 2:2\\]
RESERVED: Dont Use"]
pub type TpccCMeminitStartR = crate::BitReader;
#[doc = "Field `tpcc_c_meminit_start` writer - 2:2\\]
RESERVED: Dont Use"]
pub type TpccCMeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie TPCC_A_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear TPCC_A_MEMINIT_DONE)"]
    #[inline(always)]
    pub fn tpcc_a_meminit_start(&self) -> TpccAMeminitStartR {
        TpccAMeminitStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tpcc_b_meminit_start(&self) -> TpccBMeminitStartR {
        TpccBMeminitStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tpcc_c_meminit_start(&self) -> TpccCMeminitStartR {
        TpccCMeminitStartR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie TPCC_A_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear TPCC_A_MEMINIT_DONE)"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_start(&mut self) -> TpccAMeminitStartW<DssTpccMeminitStartSpec> {
        TpccAMeminitStartW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_meminit_start(&mut self) -> TpccBMeminitStartW<DssTpccMeminitStartSpec> {
        TpccBMeminitStartW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_c_meminit_start(&mut self) -> TpccCMeminitStartW<DssTpccMeminitStartSpec> {
        TpccCMeminitStartW::new(self, 2)
    }
}
#[doc = "DSS_TPCC_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_meminit_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_meminit_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTpccMeminitStartSpec;
impl crate::RegisterSpec for DssTpccMeminitStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tpcc_meminit_start::R`](R) reader structure"]
impl crate::Readable for DssTpccMeminitStartSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_tpcc_meminit_start::W`](W) writer structure"]
impl crate::Writable for DssTpccMeminitStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPCC_MEMINIT_START to value 0"]
impl crate::Resettable for DssTpccMeminitStartSpec {
    const RESET_VALUE: u32 = 0;
}

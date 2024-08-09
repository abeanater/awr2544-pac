#[doc = "Register `DSS_L3RAM_MEMINIT_START` reader"]
pub type R = crate::R<DssL3ramMeminitStartSpec>;
#[doc = "Register `DSS_L3RAM_MEMINIT_START` writer"]
pub type W = crate::W<DssL3ramMeminitStartSpec>;
#[doc = "Field `l3ram0_meminit_start` reader - 0:0\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
pub type L3ram0MeminitStartR = crate::BitReader;
#[doc = "Field `l3ram0_meminit_start` writer - 0:0\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
pub type L3ram0MeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram1_meminit_start` reader - 1:1\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
pub type L3ram1MeminitStartR = crate::BitReader;
#[doc = "Field `l3ram1_meminit_start` writer - 1:1\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
pub type L3ram1MeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram2_meminit_start` reader - 2:2\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
pub type L3ram2MeminitStartR = crate::BitReader;
#[doc = "Field `l3ram2_meminit_start` writer - 2:2\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
pub type L3ram2MeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram3_meminit_start` reader - 3:3\\]
Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
pub type L3ram3MeminitStartR = crate::BitReader;
#[doc = "Field `l3ram3_meminit_start` writer - 3:3\\]
Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
pub type L3ram3MeminitStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
    #[inline(always)]
    pub fn l3ram0_meminit_start(&self) -> L3ram0MeminitStartR {
        L3ram0MeminitStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
    #[inline(always)]
    pub fn l3ram1_meminit_start(&self) -> L3ram1MeminitStartR {
        L3ram1MeminitStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
    #[inline(always)]
    pub fn l3ram2_meminit_start(&self) -> L3ram2MeminitStartR {
        L3ram2MeminitStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
    #[inline(always)]
    pub fn l3ram3_meminit_start(&self) -> L3ram3MeminitStartR {
        L3ram3MeminitStartR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
    #[inline(always)]
    #[must_use]
    pub fn l3ram0_meminit_start(&mut self) -> L3ram0MeminitStartW<DssL3ramMeminitStartSpec> {
        L3ram0MeminitStartW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
    #[inline(always)]
    #[must_use]
    pub fn l3ram1_meminit_start(&mut self) -> L3ram1MeminitStartW<DssL3ramMeminitStartSpec> {
        L3ram1MeminitStartW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write pulse bit field: Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
    #[inline(always)]
    #[must_use]
    pub fn l3ram2_meminit_start(&mut self) -> L3ram2MeminitStartW<DssL3ramMeminitStartSpec> {
        L3ram2MeminitStartW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Start Memory intialization of TPCC A Param memory. Write 0x1 to start memory initilization. Write 0x0 after ensuring Memory intilization is in progress or has completed. Before starting new initilzation sequence ensure that there is no initilization sequence is in progress (ie L3RAM_MEMINIT_STATUS should be 0x0) and clear any previouls complettion status(ie write 0x1 to clear L3RAM_MEMINIT_DONE)"]
    #[inline(always)]
    #[must_use]
    pub fn l3ram3_meminit_start(&mut self) -> L3ram3MeminitStartW<DssL3ramMeminitStartSpec> {
        L3ram3MeminitStartW::new(self, 3)
    }
}
#[doc = "DSS_L3RAM_MEMINIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3ram_meminit_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3ram_meminit_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssL3ramMeminitStartSpec;
impl crate::RegisterSpec for DssL3ramMeminitStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_l3ram_meminit_start::R`](R) reader structure"]
impl crate::Readable for DssL3ramMeminitStartSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_l3ram_meminit_start::W`](W) writer structure"]
impl crate::Writable for DssL3ramMeminitStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_L3RAM_MEMINIT_START to value 0"]
impl crate::Resettable for DssL3ramMeminitStartSpec {
    const RESET_VALUE: u32 = 0;
}

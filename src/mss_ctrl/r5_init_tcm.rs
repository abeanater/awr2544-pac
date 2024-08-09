#[doc = "Register `R5_INIT_TCM` reader"]
pub type R = crate::R<R5InitTcmSpec>;
#[doc = "Register `R5_INIT_TCM` writer"]
pub type W = crate::W<R5InitTcmSpec>;
#[doc = "Field `tcma_cpu0` reader - 2:0\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables ATCM interface out of reset"]
pub type TcmaCpu0R = crate::FieldReader;
#[doc = "Field `tcma_cpu0` writer - 2:0\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables ATCM interface out of reset"]
pub type TcmaCpu0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tcmb_cpu0` reader - 6:4\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables BTCM interface out of reset"]
pub type TcmbCpu0R = crate::FieldReader;
#[doc = "Field `tcmb_cpu0` writer - 6:4\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables BTCM interface out of reset"]
pub type TcmbCpu0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `lockzram_cpu0` reader - 10:8\\]
Ti internal Register. Modifying this register is not recommended When HIGH ATCM base address at reset is 0x0 when LOW BTCM base address at reset is 0x0"]
pub type LockzramCpu0R = crate::FieldReader;
#[doc = "Field `lockzram_cpu0` writer - 10:8\\]
Ti internal Register. Modifying this register is not recommended When HIGH ATCM base address at reset is 0x0 when LOW BTCM base address at reset is 0x0"]
pub type LockzramCpu0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tcma_cpu1` reader - 14:12\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables ATCM interface out of reset"]
pub type TcmaCpu1R = crate::FieldReader;
#[doc = "Field `tcma_cpu1` writer - 14:12\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables ATCM interface out of reset"]
pub type TcmaCpu1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `tcmb_cpu1` reader - 18:16\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables BTCM interface out of reset"]
pub type TcmbCpu1R = crate::FieldReader;
#[doc = "Field `tcmb_cpu1` writer - 18:16\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables BTCM interface out of reset"]
pub type TcmbCpu1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `lockzram_cpu1` reader - 22:20\\]
Ti internal Register. Modifying this register is not recommended When HIGH ATCM base address at reset is 0x0 when LOW BTCM base address at reset is 0x0"]
pub type LockzramCpu1R = crate::FieldReader;
#[doc = "Field `lockzram_cpu1` writer - 22:20\\]
Ti internal Register. Modifying this register is not recommended When HIGH ATCM base address at reset is 0x0 when LOW BTCM base address at reset is 0x0"]
pub type LockzramCpu1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables ATCM interface out of reset"]
    #[inline(always)]
    pub fn tcma_cpu0(&self) -> TcmaCpu0R {
        TcmaCpu0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables BTCM interface out of reset"]
    #[inline(always)]
    pub fn tcmb_cpu0(&self) -> TcmbCpu0R {
        TcmbCpu0R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Ti internal Register. Modifying this register is not recommended When HIGH ATCM base address at reset is 0x0 when LOW BTCM base address at reset is 0x0"]
    #[inline(always)]
    pub fn lockzram_cpu0(&self) -> LockzramCpu0R {
        LockzramCpu0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables ATCM interface out of reset"]
    #[inline(always)]
    pub fn tcma_cpu1(&self) -> TcmaCpu1R {
        TcmaCpu1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables BTCM interface out of reset"]
    #[inline(always)]
    pub fn tcmb_cpu1(&self) -> TcmbCpu1R {
        TcmbCpu1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Ti internal Register. Modifying this register is not recommended When HIGH ATCM base address at reset is 0x0 when LOW BTCM base address at reset is 0x0"]
    #[inline(always)]
    pub fn lockzram_cpu1(&self) -> LockzramCpu1R {
        LockzramCpu1R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables ATCM interface out of reset"]
    #[inline(always)]
    #[must_use]
    pub fn tcma_cpu0(&mut self) -> TcmaCpu0W<R5InitTcmSpec> {
        TcmaCpu0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables BTCM interface out of reset"]
    #[inline(always)]
    #[must_use]
    pub fn tcmb_cpu0(&mut self) -> TcmbCpu0W<R5InitTcmSpec> {
        TcmbCpu0W::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Ti internal Register. Modifying this register is not recommended When HIGH ATCM base address at reset is 0x0 when LOW BTCM base address at reset is 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn lockzram_cpu0(&mut self) -> LockzramCpu0W<R5InitTcmSpec> {
        LockzramCpu0W::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables ATCM interface out of reset"]
    #[inline(always)]
    #[must_use]
    pub fn tcma_cpu1(&mut self) -> TcmaCpu1W<R5InitTcmSpec> {
        TcmaCpu1W::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Ti internal Register. Modifying this register is not recommended When HIGH enables BTCM interface out of reset"]
    #[inline(always)]
    #[must_use]
    pub fn tcmb_cpu1(&mut self) -> TcmbCpu1W<R5InitTcmSpec> {
        TcmbCpu1W::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Ti internal Register. Modifying this register is not recommended When HIGH ATCM base address at reset is 0x0 when LOW BTCM base address at reset is 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn lockzram_cpu1(&mut self) -> LockzramCpu1W<R5InitTcmSpec> {
        LockzramCpu1W::new(self, 20)
    }
}
#[doc = "R5_INIT_TCM\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_init_tcm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_init_tcm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5InitTcmSpec;
impl crate::RegisterSpec for R5InitTcmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_init_tcm::R`](R) reader structure"]
impl crate::Readable for R5InitTcmSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_init_tcm::W`](W) writer structure"]
impl crate::Writable for R5InitTcmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_INIT_TCM to value 0"]
impl crate::Resettable for R5InitTcmSpec {
    const RESET_VALUE: u32 = 0;
}

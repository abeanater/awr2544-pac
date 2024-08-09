#[doc = "Register `MSS_L2_MEM_INIT` reader"]
pub type R = crate::R<MssL2MemInitSpec>;
#[doc = "Register `MSS_L2_MEM_INIT` writer"]
pub type W = crate::W<MssL2MemInitSpec>;
#[doc = "Field `partition0` reader - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank0. Value in each row is initialized to 0x0"]
pub type Partition0R = crate::BitReader;
#[doc = "Field `partition0` writer - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank0. Value in each row is initialized to 0x0"]
pub type Partition0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `partition1` reader - 1:1\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank1. Value in each row is initialized to 0x0"]
pub type Partition1R = crate::BitReader;
#[doc = "Field `partition1` writer - 1:1\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank1. Value in each row is initialized to 0x0"]
pub type Partition1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `partition2` reader - 2:2\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank2. Value in each row is initialized to 0x0"]
pub type Partition2R = crate::BitReader;
#[doc = "Field `partition2` writer - 2:2\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank2. Value in each row is initialized to 0x0"]
pub type Partition2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank0. Value in each row is initialized to 0x0"]
    #[inline(always)]
    pub fn partition0(&self) -> Partition0R {
        Partition0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank1. Value in each row is initialized to 0x0"]
    #[inline(always)]
    pub fn partition1(&self) -> Partition1R {
        Partition1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank2. Value in each row is initialized to 0x0"]
    #[inline(always)]
    pub fn partition2(&self) -> Partition2R {
        Partition2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank0. Value in each row is initialized to 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn partition0(&mut self) -> Partition0W<MssL2MemInitSpec> {
        Partition0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank1. Value in each row is initialized to 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn partition1(&mut self) -> Partition1W<MssL2MemInitSpec> {
        Partition1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write_pulse bit field: Writing 1'b1 will start initializing the L2 Bank2. Value in each row is initialized to 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn partition2(&mut self) -> Partition2W<MssL2MemInitSpec> {
        Partition2W::new(self, 2)
    }
}
#[doc = "MSS_L2_MEM_INIT\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_l2_mem_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_l2_mem_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssL2MemInitSpec;
impl crate::RegisterSpec for MssL2MemInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_l2_mem_init::R`](R) reader structure"]
impl crate::Readable for MssL2MemInitSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_l2_mem_init::W`](W) writer structure"]
impl crate::Writable for MssL2MemInitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_L2_MEM_INIT to value 0"]
impl crate::Resettable for MssL2MemInitSpec {
    const RESET_VALUE: u32 = 0;
}

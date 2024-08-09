#[doc = "Register `MEM_INIT_STATUS` reader"]
pub type R = crate::R<MemInitStatusSpec>;
#[doc = "Register `MEM_INIT_STATUS` writer"]
pub type W = crate::W<MemInitStatusSpec>;
#[doc = "Field `dmem0` reader - 0:0\\]
Will be 1'b1 during memory initialization for dmem0"]
pub type Dmem0R = crate::BitReader;
#[doc = "Field `dmem0` writer - 0:0\\]
Will be 1'b1 during memory initialization for dmem0"]
pub type Dmem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem1` reader - 1:1\\]
Will be 1'b1 during memory initialization for dmem1"]
pub type Dmem1R = crate::BitReader;
#[doc = "Field `dmem1` writer - 1:1\\]
Will be 1'b1 during memory initialization for dmem1"]
pub type Dmem1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem2` reader - 2:2\\]
Will be 1'b1 during memory initialization for dmem2"]
pub type Dmem2R = crate::BitReader;
#[doc = "Field `dmem2` writer - 2:2\\]
Will be 1'b1 during memory initialization for dmem2"]
pub type Dmem2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem3` reader - 3:3\\]
Will be 1'b1 during memory initialization for dmem3"]
pub type Dmem3R = crate::BitReader;
#[doc = "Field `dmem3` writer - 3:3\\]
Will be 1'b1 during memory initialization for dmem3"]
pub type Dmem3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `param_ram` reader - 8:8\\]
Will be 1'b1 during memory initialization for param_ram"]
pub type ParamRamR = crate::BitReader;
#[doc = "Field `param_ram` writer - 8:8\\]
Will be 1'b1 during memory initialization for param_ram"]
pub type ParamRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `window_ram` reader - 9:9\\]
Will be 1'b1 during memory initialization for window_ram"]
pub type WindowRamR = crate::BitReader;
#[doc = "Field `window_ram` writer - 9:9\\]
Will be 1'b1 during memory initialization for window_ram"]
pub type WindowRamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Will be 1'b1 during memory initialization for dmem0"]
    #[inline(always)]
    pub fn dmem0(&self) -> Dmem0R {
        Dmem0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Will be 1'b1 during memory initialization for dmem1"]
    #[inline(always)]
    pub fn dmem1(&self) -> Dmem1R {
        Dmem1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Will be 1'b1 during memory initialization for dmem2"]
    #[inline(always)]
    pub fn dmem2(&self) -> Dmem2R {
        Dmem2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Will be 1'b1 during memory initialization for dmem3"]
    #[inline(always)]
    pub fn dmem3(&self) -> Dmem3R {
        Dmem3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Will be 1'b1 during memory initialization for param_ram"]
    #[inline(always)]
    pub fn param_ram(&self) -> ParamRamR {
        ParamRamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Will be 1'b1 during memory initialization for window_ram"]
    #[inline(always)]
    pub fn window_ram(&self) -> WindowRamR {
        WindowRamR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Will be 1'b1 during memory initialization for dmem0"]
    #[inline(always)]
    #[must_use]
    pub fn dmem0(&mut self) -> Dmem0W<MemInitStatusSpec> {
        Dmem0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Will be 1'b1 during memory initialization for dmem1"]
    #[inline(always)]
    #[must_use]
    pub fn dmem1(&mut self) -> Dmem1W<MemInitStatusSpec> {
        Dmem1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Will be 1'b1 during memory initialization for dmem2"]
    #[inline(always)]
    #[must_use]
    pub fn dmem2(&mut self) -> Dmem2W<MemInitStatusSpec> {
        Dmem2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Will be 1'b1 during memory initialization for dmem3"]
    #[inline(always)]
    #[must_use]
    pub fn dmem3(&mut self) -> Dmem3W<MemInitStatusSpec> {
        Dmem3W::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
Will be 1'b1 during memory initialization for param_ram"]
    #[inline(always)]
    #[must_use]
    pub fn param_ram(&mut self) -> ParamRamW<MemInitStatusSpec> {
        ParamRamW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Will be 1'b1 during memory initialization for window_ram"]
    #[inline(always)]
    #[must_use]
    pub fn window_ram(&mut self) -> WindowRamW<MemInitStatusSpec> {
        WindowRamW::new(self, 9)
    }
}
#[doc = "MEM_INIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_init_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_init_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemInitStatusSpec;
impl crate::RegisterSpec for MemInitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_init_status::R`](R) reader structure"]
impl crate::Readable for MemInitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_init_status::W`](W) writer structure"]
impl crate::Writable for MemInitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_INIT_STATUS to value 0"]
impl crate::Resettable for MemInitStatusSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MEM_INIT_START` reader"]
pub type R = crate::R<MemInitStartSpec>;
#[doc = "Register `MEM_INIT_START` writer"]
pub type W = crate::W<MemInitStartSpec>;
#[doc = "Field `dmem0` reader - 0:0\\]
writing 1'b1 would start the memory initialization for the DMEM0 It s a self clearing bit"]
pub type Dmem0R = crate::BitReader;
#[doc = "Field `dmem0` writer - 0:0\\]
writing 1'b1 would start the memory initialization for the DMEM0 It s a self clearing bit"]
pub type Dmem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem1` reader - 1:1\\]
writing 1'b1 would start the memory initialization for the DMEM1. It s a self clearing bit"]
pub type Dmem1R = crate::BitReader;
#[doc = "Field `dmem1` writer - 1:1\\]
writing 1'b1 would start the memory initialization for the DMEM1. It s a self clearing bit"]
pub type Dmem1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem2` reader - 2:2\\]
writing 1'b1 would start the memory initialization for the DMEM2 It s a self clearing bit"]
pub type Dmem2R = crate::BitReader;
#[doc = "Field `dmem2` writer - 2:2\\]
writing 1'b1 would start the memory initialization for the DMEM2 It s a self clearing bit"]
pub type Dmem2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dmem3` reader - 3:3\\]
writing 1'b1 would start the memory initialization for the DMEM3 It s a self clearing bit"]
pub type Dmem3R = crate::BitReader;
#[doc = "Field `dmem3` writer - 3:3\\]
writing 1'b1 would start the memory initialization for the DMEM3 It s a self clearing bit"]
pub type Dmem3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `param_ram` reader - 8:8\\]
writing 1'b1 would start the memory initialization for the Param memory It s a self clearing bit"]
pub type ParamRamR = crate::BitReader;
#[doc = "Field `param_ram` writer - 8:8\\]
writing 1'b1 would start the memory initialization for the Param memory It s a self clearing bit"]
pub type ParamRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `window_ram` reader - 9:9\\]
writing 1'b1 would start the memory initialization for the window memory It s a self clearing bit"]
pub type WindowRamR = crate::BitReader;
#[doc = "Field `window_ram` writer - 9:9\\]
writing 1'b1 would start the memory initialization for the window memory It s a self clearing bit"]
pub type WindowRamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 would start the memory initialization for the DMEM0 It s a self clearing bit"]
    #[inline(always)]
    pub fn dmem0(&self) -> Dmem0R {
        Dmem0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
writing 1'b1 would start the memory initialization for the DMEM1. It s a self clearing bit"]
    #[inline(always)]
    pub fn dmem1(&self) -> Dmem1R {
        Dmem1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
writing 1'b1 would start the memory initialization for the DMEM2 It s a self clearing bit"]
    #[inline(always)]
    pub fn dmem2(&self) -> Dmem2R {
        Dmem2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
writing 1'b1 would start the memory initialization for the DMEM3 It s a self clearing bit"]
    #[inline(always)]
    pub fn dmem3(&self) -> Dmem3R {
        Dmem3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
writing 1'b1 would start the memory initialization for the Param memory It s a self clearing bit"]
    #[inline(always)]
    pub fn param_ram(&self) -> ParamRamR {
        ParamRamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
writing 1'b1 would start the memory initialization for the window memory It s a self clearing bit"]
    #[inline(always)]
    pub fn window_ram(&self) -> WindowRamR {
        WindowRamR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 would start the memory initialization for the DMEM0 It s a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmem0(&mut self) -> Dmem0W<MemInitStartSpec> {
        Dmem0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
writing 1'b1 would start the memory initialization for the DMEM1. It s a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmem1(&mut self) -> Dmem1W<MemInitStartSpec> {
        Dmem1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
writing 1'b1 would start the memory initialization for the DMEM2 It s a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmem2(&mut self) -> Dmem2W<MemInitStartSpec> {
        Dmem2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
writing 1'b1 would start the memory initialization for the DMEM3 It s a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmem3(&mut self) -> Dmem3W<MemInitStartSpec> {
        Dmem3W::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
writing 1'b1 would start the memory initialization for the Param memory It s a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn param_ram(&mut self) -> ParamRamW<MemInitStartSpec> {
        ParamRamW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
writing 1'b1 would start the memory initialization for the window memory It s a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn window_ram(&mut self) -> WindowRamW<MemInitStartSpec> {
        WindowRamW::new(self, 9)
    }
}
#[doc = "MEM_INIT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_init_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_init_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemInitStartSpec;
impl crate::RegisterSpec for MemInitStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_init_start::R`](R) reader structure"]
impl crate::Readable for MemInitStartSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_init_start::W`](W) writer structure"]
impl crate::Writable for MemInitStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_INIT_START to value 0"]
impl crate::Resettable for MemInitStartSpec {
    const RESET_VALUE: u32 = 0;
}

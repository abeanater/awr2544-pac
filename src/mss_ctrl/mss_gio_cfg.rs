#[doc = "Register `MSS_GIO_CFG` reader"]
pub type R = crate::R<MssGioCfgSpec>;
#[doc = "Register `MSS_GIO_CFG` writer"]
pub type W = crate::W<MssGioCfgSpec>;
#[doc = "Field `gio_config` reader - 31:0\\]
bit0 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT0 to IRQ bit1 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT1to IRQ bit2: writing '1' will slect negedge for pulse generation of GIO_PAD_INT2 to IRQ bit3 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT3 to IRQ bit4 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT4 to IRQ bit5 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT5 to IRQ bit6 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT6 to IRQ bit7 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT7 to IRQ"]
pub type GioConfigR = crate::FieldReader<u32>;
#[doc = "Field `gio_config` writer - 31:0\\]
bit0 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT0 to IRQ bit1 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT1to IRQ bit2: writing '1' will slect negedge for pulse generation of GIO_PAD_INT2 to IRQ bit3 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT3 to IRQ bit4 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT4 to IRQ bit5 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT5 to IRQ bit6 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT6 to IRQ bit7 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT7 to IRQ"]
pub type GioConfigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
bit0 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT0 to IRQ bit1 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT1to IRQ bit2: writing '1' will slect negedge for pulse generation of GIO_PAD_INT2 to IRQ bit3 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT3 to IRQ bit4 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT4 to IRQ bit5 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT5 to IRQ bit6 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT6 to IRQ bit7 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT7 to IRQ"]
    #[inline(always)]
    pub fn gio_config(&self) -> GioConfigR {
        GioConfigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
bit0 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT0 to IRQ bit1 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT1to IRQ bit2: writing '1' will slect negedge for pulse generation of GIO_PAD_INT2 to IRQ bit3 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT3 to IRQ bit4 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT4 to IRQ bit5 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT5 to IRQ bit6 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT6 to IRQ bit7 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT7 to IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn gio_config(&mut self) -> GioConfigW<MssGioCfgSpec> {
        GioConfigW::new(self, 0)
    }
}
#[doc = "MSS_GIO_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_gio_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_gio_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssGioCfgSpec;
impl crate::RegisterSpec for MssGioCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_gio_cfg::R`](R) reader structure"]
impl crate::Readable for MssGioCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_gio_cfg::W`](W) writer structure"]
impl crate::Writable for MssGioCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_GIO_CFG to value 0"]
impl crate::Resettable for MssGioCfgSpec {
    const RESET_VALUE: u32 = 0;
}

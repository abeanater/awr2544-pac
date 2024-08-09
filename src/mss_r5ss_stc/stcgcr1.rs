#[doc = "Register `STCGCR1` reader"]
pub type R = crate::R<Stcgcr1Spec>;
#[doc = "Register `STCGCR1` writer"]
pub type W = crate::W<Stcgcr1Spec>;
#[doc = "Field `ST_ENA_B4` reader - 3:0\\]
Self test enable key (RWP - Read, Priviledge Mode Write only) 1010 = Self test run enabled All values other than 1010 = Self test run disabled"]
pub type StEnaB4R = crate::FieldReader;
#[doc = "Field `ST_ENA_B4` writer - 3:0\\]
Self test enable key (RWP - Read, Priviledge Mode Write only) 1010 = Self test run enabled All values other than 1010 = Self test run disabled"]
pub type StEnaB4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROM_ACCESS_INV` reader - 4:4\\]
Rom access inversion mode (RWP - Read, Priviledge Mode Write only) - NOT SUPPORTED"]
pub type RomAccessInvR = crate::BitReader;
#[doc = "Field `ROM_ACCESS_INV` writer - 4:4\\]
Rom access inversion mode (RWP - Read, Priviledge Mode Write only) - NOT SUPPORTED"]
pub type RomAccessInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SCAN_MODE` reader - 5:5\\]
LP scan mode (RWP - Read, Priviledge Mode Write only) This bit is used to decide the scan configuration: 1 = Operates in Low Power Scan Mode. 0 = Operates in Normal Scan Mode."]
pub type LpScanModeR = crate::BitReader;
#[doc = "Field `LP_SCAN_MODE` writer - 5:5\\]
LP scan mode (RWP - Read, Priviledge Mode Write only) This bit is used to decide the scan configuration: 1 = Operates in Low Power Scan Mode. 0 = Operates in Normal Scan Mode."]
pub type LpScanModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODEC_SPREAD_MODE` reader - 6:6\\]
Codec Spread Mode control signal (RWP - Read, Priviledge Mode Write only) This bit is used to configure the codec in spread / X-OR mode. 1 = Spread mode 0 = XOR mode"]
pub type CodecSpreadModeR = crate::BitReader;
#[doc = "Field `CODEC_SPREAD_MODE` writer - 6:6\\]
Codec Spread Mode control signal (RWP - Read, Priviledge Mode Write only) This bit is used to configure the codec in spread / X-OR mode. 1 = Spread mode 0 = XOR mode"]
pub type CodecSpreadModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 7:7\\]
Reserved bits"]
pub type Nu3R = crate::BitReader;
#[doc = "Field `NU3` writer - 7:7\\]
Reserved bits"]
pub type Nu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEG0_CORE_SEL` reader - 11:8\\]
Selects the Segment0 CORE for self test (RWP - Read, Priviledge Mode Write only) Select the Segment0 CORE for Self -Test 0001 = Select CORE for selftest Other = CORE not selected."]
pub type Seg0CoreSelR = crate::FieldReader;
#[doc = "Field `SEG0_CORE_SEL` writer - 11:8\\]
Selects the Segment0 CORE for self test (RWP - Read, Priviledge Mode Write only) Select the Segment0 CORE for Self -Test 0001 = Select CORE for selftest Other = CORE not selected."]
pub type Seg0CoreSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU2` reader - 31:12\\]
Reserved bits"]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - 31:12\\]
Reserved bits"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Self test enable key (RWP - Read, Priviledge Mode Write only) 1010 = Self test run enabled All values other than 1010 = Self test run disabled"]
    #[inline(always)]
    pub fn st_ena_b4(&self) -> StEnaB4R {
        StEnaB4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Rom access inversion mode (RWP - Read, Priviledge Mode Write only) - NOT SUPPORTED"]
    #[inline(always)]
    pub fn rom_access_inv(&self) -> RomAccessInvR {
        RomAccessInvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
LP scan mode (RWP - Read, Priviledge Mode Write only) This bit is used to decide the scan configuration: 1 = Operates in Low Power Scan Mode. 0 = Operates in Normal Scan Mode."]
    #[inline(always)]
    pub fn lp_scan_mode(&self) -> LpScanModeR {
        LpScanModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Codec Spread Mode control signal (RWP - Read, Priviledge Mode Write only) This bit is used to configure the codec in spread / X-OR mode. 1 = Spread mode 0 = XOR mode"]
    #[inline(always)]
    pub fn codec_spread_mode(&self) -> CodecSpreadModeR {
        CodecSpreadModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Selects the Segment0 CORE for self test (RWP - Read, Priviledge Mode Write only) Select the Segment0 CORE for Self -Test 0001 = Select CORE for selftest Other = CORE not selected."]
    #[inline(always)]
    pub fn seg0_core_sel(&self) -> Seg0CoreSelR {
        Seg0CoreSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Self test enable key (RWP - Read, Priviledge Mode Write only) 1010 = Self test run enabled All values other than 1010 = Self test run disabled"]
    #[inline(always)]
    #[must_use]
    pub fn st_ena_b4(&mut self) -> StEnaB4W<Stcgcr1Spec> {
        StEnaB4W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Rom access inversion mode (RWP - Read, Priviledge Mode Write only) - NOT SUPPORTED"]
    #[inline(always)]
    #[must_use]
    pub fn rom_access_inv(&mut self) -> RomAccessInvW<Stcgcr1Spec> {
        RomAccessInvW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
LP scan mode (RWP - Read, Priviledge Mode Write only) This bit is used to decide the scan configuration: 1 = Operates in Low Power Scan Mode. 0 = Operates in Normal Scan Mode."]
    #[inline(always)]
    #[must_use]
    pub fn lp_scan_mode(&mut self) -> LpScanModeW<Stcgcr1Spec> {
        LpScanModeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Codec Spread Mode control signal (RWP - Read, Priviledge Mode Write only) This bit is used to configure the codec in spread / X-OR mode. 1 = Spread mode 0 = XOR mode"]
    #[inline(always)]
    #[must_use]
    pub fn codec_spread_mode(&mut self) -> CodecSpreadModeW<Stcgcr1Spec> {
        CodecSpreadModeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<Stcgcr1Spec> {
        Nu3W::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Selects the Segment0 CORE for self test (RWP - Read, Priviledge Mode Write only) Select the Segment0 CORE for Self -Test 0001 = Select CORE for selftest Other = CORE not selected."]
    #[inline(always)]
    #[must_use]
    pub fn seg0_core_sel(&mut self) -> Seg0CoreSelW<Stcgcr1Spec> {
        Seg0CoreSelW::new(self, 8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Stcgcr1Spec> {
        Nu2W::new(self, 12)
    }
}
#[doc = "Self test Global control Reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`stcgcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcgcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stcgcr1Spec;
impl crate::RegisterSpec for Stcgcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcgcr1::R`](R) reader structure"]
impl crate::Readable for Stcgcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`stcgcr1::W`](W) writer structure"]
impl crate::Writable for Stcgcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCGCR1 to value 0"]
impl crate::Resettable for Stcgcr1Spec {
    const RESET_VALUE: u32 = 0;
}

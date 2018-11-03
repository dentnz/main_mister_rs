use cores::api::{
    Core,
    KEY_CODE_NONE as NONE,
    KEY_CAPS_TOGGLE as CAPS_TOGGLE
};

const MINIMIG_KEYMAP : &'static [u32] = &[
    NONE, //0   KEY_RESERVED	
	0x45, //1   KEY_ESC			
	0x01, //2   KEY_1			
	0x02, //3   KEY_2			
	0x03, //4   KEY_3			
	0x04, //5   KEY_4			
	0x05, //6   KEY_5			
	0x06, //7   KEY_6			
	0x07, //8   KEY_7			
	0x08, //9   KEY_8			
	0x09, //10  KEY_9			
	0x0a, //11  KEY_0			
	0x0b, //12  KEY_MINUS		
	0x0c, //13  KEY_EQUAL		
	0x41, //14  KEY_BACKSPACE	
	0x42, //15  KEY_TAB			
	0x10, //16  KEY_Q			
	0x11, //17  KEY_W			
	0x12, //18  KEY_E			
	0x13, //19  KEY_R			
	0x14, //20  KEY_T			
	0x15, //21  KEY_Y			
	0x16, //22  KEY_U			
	0x17, //23  KEY_I			
	0x18, //24  KEY_O			
	0x19, //25  KEY_P			
	0x1a, //26  KEY_LEFTBRACE	
	0x1b, //27  KEY_RIGHTBRACE	
	0x44, //28  KEY_ENTER		
	0x63, //29  KEY_LEFTCTRL	
	0x20, //30  KEY_A			
	0x21, //31  KEY_S			
	0x22, //32  KEY_D			
	0x23, //33  KEY_F			
	0x24, //34  KEY_G			
	0x25, //35  KEY_H			
	0x26, //36  KEY_J			
	0x27, //37  KEY_K			
	0x28, //38  KEY_L			
	0x29, //39  KEY_SEMICOLON	
	0x2a, //40  KEY_APOSTROPHE	
	0x00, //41  KEY_GRAVE		
	0x60, //42  KEY_LEFTSHIFT	
	0x0d, //43  KEY_BACKSLASH	
	0x31, //44  KEY_Z			
	0x32, //45  KEY_X			
	0x33, //46  KEY_C			
	0x34, //47  KEY_V			
	0x35, //48  KEY_B			
	0x36, //49  KEY_N			
	0x37, //50  KEY_M			
	0x38, //51  KEY_COMMA		
	0x39, //52  KEY_DOT			
	0x3a, //53  KEY_SLASH		
	0x61, //54  KEY_RIGHTSHIFT	
	0x5d, //55  KEY_KPASTERISK	
	0x64, //56  KEY_LEFTALT		
	0x40, //57  KEY_SPACE		
	0x62 | CAPS_TOGGLE, //58  KEY_CAPSLOCK	
	0x50, //59  KEY_F1			
	0x51, //60  KEY_F2			
	0x52, //61  KEY_F3			
	0x53, //62  KEY_F4			
	0x54, //63  KEY_F5			
	0x55, //64  KEY_F6			
	0x56, //65  KEY_F7			
	0x57, //66  KEY_F8			
	0x58, //67  KEY_F9			
	0x59, //68  KEY_F10			
	NONE, //69  KEY_NUMLOCK		
	NONE, //70  KEY_SCROLLLOCK	
	0x3d, //71  KEY_KP7			
	0x3e, //72  KEY_KP8			
	0x3f, //73  KEY_KP9			
	0x4a, //74  KEY_KPMINUS		
	0x2d, //75  KEY_KP4			
	0x2e, //76  KEY_KP5			
	0x2f, //77  KEY_KP6			
	0x5e, //78  KEY_KPPLUS		
	0x1d, //79  KEY_KP1			
	0x1e, //80  KEY_KP2			
	0x1f, //81  KEY_KP3			
	0x0f, //82  KEY_KP0			
	0x3c, //83  KEY_KPDOT		
	NONE, //84  ???				
	NONE, //85  KEY_ZENKAKU		
	NONE, //86  KEY_102ND		
	0x5f, //87  KEY_F11			
	NONE, //88  KEY_F12			
	NONE, //89  KEY_RO			
	NONE, //90  KEY_KATAKANA	
	NONE, //91  KEY_HIRAGANA	
	NONE, //92  KEY_HENKAN		
	NONE, //93  KEY_KATAKANA	
	NONE, //94  KEY_MUHENKAN	
	NONE, //95  KEY_KPJPCOMMA	
	0x43, //96  KEY_KPENTER		
	0x63, //97  KEY_RIGHTCTRL	
	0x5c, //98  KEY_KPSLASH		
	NONE, //99  KEY_SYSRQ		
	0x65, //100 KEY_RIGHTALT	
	NONE, //101 KEY_LINEFEED	
	0x6a, //102 KEY_HOME		
	0x4c, //103 KEY_UP			
	NONE, //104 KEY_PAGEUP		
	0x4f, //105 KEY_LEFT		
	0x4e, //106 KEY_RIGHT		
	NONE, //107 KEY_END			
	0x4d, //108 KEY_DOWN		
	NONE, //109 KEY_PAGEDOWN	
	0x0d, //110 KEY_INSERT		
	0x46, //111 KEY_DELETE		
	NONE, //112 KEY_MACRO		
	NONE, //113 KEY_MUTE		
	NONE, //114 KEY_VOLUMEDOWN	
	NONE, //115 KEY_VOLUMEUP	
	NONE, //116 KEY_POWER		
	NONE, //117 KEY_KPEQUAL		
	NONE, //118 KEY_KPPLUSMINUS	
	NONE, //119 KEY_PAUSE		
	NONE, //120 KEY_SCALE		
	NONE, //121 KEY_KPCOMMA		
	NONE, //122 KEY_HANGEUL		
	NONE, //123 KEY_HANJA		
	NONE, //124 KEY_YEN			
	0x66, //125 KEY_LEFTMETA	
	0x67, //126 KEY_RIGHTMETA	
	NONE, //127 KEY_COMPOSE		
	NONE, //128 KEY_STOP		
	NONE, //129 KEY_AGAIN		
	NONE, //130 KEY_PROPS		
	NONE, //131 KEY_UNDO		
	NONE, //132 KEY_FRONT		
	NONE, //133 KEY_COPY		
	NONE, //134 KEY_OPEN		
	NONE, //135 KEY_PASTE		
	NONE, //136 KEY_FIND		
	NONE, //137 KEY_CUT			
	NONE, //138 KEY_HELP		
	NONE, //139 KEY_MENU		
	NONE, //140 KEY_CALC		
	NONE, //141 KEY_SETUP		
	NONE, //142 KEY_SLEEP		
	NONE, //143 KEY_WAKEUP		
	NONE, //144 KEY_FILE		
	NONE, //145 KEY_SENDFILE	
	NONE, //146 KEY_DELETEFILE	
	NONE, //147 KEY_XFER		
	NONE, //148 KEY_PROG1		
	NONE, //149 KEY_PROG2		
	NONE, //150 KEY_WWW			
	NONE, //151 KEY_MSDOS		
	NONE, //152 KEY_SCREENLOCK	
	NONE, //153 KEY_DIRECTION	
	NONE, //154 KEY_CYCLEWINDOWS
	NONE, //155 KEY_MAIL		
	NONE, //156 KEY_BOOKMARKS	
	NONE, //157 KEY_COMPUTER	
	NONE, //158 KEY_BACK		
	NONE, //159 KEY_FORWARD		
	NONE, //160 KEY_CLOSECD		
	NONE, //161 KEY_EJECTCD		
	NONE, //162 KEY_EJECTCLOSECD
	NONE, //163 KEY_NEXTSONG	
	NONE, //164 KEY_PLAYPAUSE	
	NONE, //165 KEY_PREVIOUSSONG
	NONE, //166 KEY_STOPCD		
	NONE, //167 KEY_RECORD		
	NONE, //168 KEY_REWIND		
	NONE, //169 KEY_PHONE		
	NONE, //170 KEY_ISO			
	NONE, //171 KEY_CONFIG		
	NONE, //172 KEY_HOMEPAGE	
	NONE, //173 KEY_REFRESH		
	NONE, //174 KEY_EXIT		
	NONE, //175 KEY_MOVE		
	NONE, //176 KEY_EDIT		
	NONE, //177 KEY_SCROLLUP	
	NONE, //178 KEY_SCROLLDOWN	
	NONE, //179 KEY_KPLEFTPAREN	
	NONE, //180 KEY_KPRIGHTPAREN
	NONE, //181 KEY_NEW			
	NONE, //182 KEY_REDO		
	0x5a, //183 KEY_F13			
	0x5b, //184 KEY_F14			
	NONE, //185 KEY_F15			
	0x5f, //186 KEY_F16			
	NONE, //187 KEY_F17			
	NONE, //188 KEY_F18			
	NONE, //189 KEY_F19			
	NONE, //190 KEY_F20			
	NONE, //191 KEY_F21			
	NONE, //192 KEY_F22			
	NONE, //193 KEY_F23			
	0x63, //194 KEY_F24			
	NONE, //195 ???				
	NONE, //196 ???				
	NONE, //197 ???				
	NONE, //198 ???				
	NONE, //199 ???				
	NONE, //200 KEY_PLAYCD		
	NONE, //201 KEY_PAUSECD		
	NONE, //202 KEY_PROG3		
	NONE, //203 KEY_PROG4		
	NONE, //204 KEY_DASHBOARD	
	NONE, //205 KEY_SUSPEND		
	NONE, //206 KEY_CLOSE		
	NONE, //207 KEY_PLAY		
	NONE, //208 KEY_FASTFORWARD	
	NONE, //209 KEY_BASSBOOST	
	NONE, //210 KEY_PRINT		
	NONE, //211 KEY_HP			
	NONE, //212 KEY_CAMERA		
	NONE, //213 KEY_SOUND		
	NONE, //214 KEY_QUESTION	
	NONE, //215 KEY_EMAIL		
	NONE, //216 KEY_CHAT		
	NONE, //217 KEY_SEARCH		
	NONE, //218 KEY_CONNECT		
	NONE, //219 KEY_FINANCE		
	NONE, //220 KEY_SPORT		
	NONE, //221 KEY_SHOP		
	NONE, //222 KEY_ALTERASE	
	NONE, //223 KEY_CANCEL		
	NONE, //224 KEY_BRIGHT_DOWN	
	NONE, //225 KEY_BRIGHT_UP	
	NONE, //226 KEY_MEDIA		
	NONE, //227 KEY_SWITCHVIDEO	
	NONE, //228 KEY_DILLUMTOGGLE
	NONE, //229 KEY_DILLUMDOWN	
	NONE, //230 KEY_DILLUMUP	
	NONE, //231 KEY_SEND		
	NONE, //232 KEY_REPLY		
	NONE, //233 KEY_FORWARDMAIL	
	NONE, //234 KEY_SAVE		
	NONE, //235 KEY_DOCUMENTS	
	NONE, //236 KEY_BATTERY		
	NONE, //237 KEY_BLUETOOTH	
	NONE, //238 KEY_WLAN		
	NONE, //239 KEY_UWB			
	NONE, //240 KEY_UNKNOWN		
	NONE, //241 KEY_VIDEO_NEXT	
	NONE, //242 KEY_VIDEO_PREV	
	NONE, //243 KEY_BRIGHT_CYCLE
	NONE, //244 KEY_BRIGHT_AUTO	
	NONE, //245 KEY_DISPLAY_OFF	
	NONE, //246 KEY_WWAN		
	NONE, //247 KEY_RFKILL		
	NONE, //248 KEY_MICMUTE		
	NONE, //249 ???				
	NONE, //250 ???				
	NONE, //251 ???				
	NONE, //252 ???				
	NONE, //253 ???				
	NONE, //254 ???				
	NONE  //255 ???				
];

const MINIMIG_CORE_ID : u8 = 0xa5;

pub struct MinimigCore();

impl MinimigCore {
    pub fn new() -> MinimigCore {   
        MinimigCore {}
    }
}

impl Core for MinimigCore {
	fn core_id(&self) -> u8 {
		MINIMIG_CORE_ID
	}

    fn map_key(&self, key: u16) -> u32 {
        match MINIMIG_KEYMAP.get(key as usize) {
            Some(code) => *code,
            None => NONE
        }
    }
}
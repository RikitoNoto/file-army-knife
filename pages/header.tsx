import {initializeIcons, Icon, Panel, PanelType, IconButton} from '@fluentui/react'
import { useBoolean } from '@fluentui/react-hooks';
import Link from 'next/link'
import headerStyles from '../styles/Header.module.css'

const Header = () => {
    const [isOpen, { setTrue: openPanel, setFalse: dismissPanel }] = useBoolean(false);
    return (
        <div>
            <Panel
                isLightDismiss
                isBlocking={false}
                isOpen={isOpen}
                onDismiss={dismissPanel}
                type={PanelType.smallFixedNear}
                hasCloseButton={false}
            >
                <div onClick={dismissPanel}>
                    <IconButton iconProps={{iconName: 'Cancel'}} title={"menu"}/>
                </div>

                <Link href="/line_counter/line_counter" className={`${headerStyles.hamburger_item}`}>
                    <Icon iconName='NumberedList' className={`${headerStyles.hamburger_item__icon}`}/>
                    Line Counter
                </Link>

                <Link href="/docment_search/docment_search" className={`${headerStyles.hamburger_item}`}>
                    <Icon iconName='DocumentSearch' className={`${headerStyles.hamburger_item__icon}`}/>
                    Search Docment
                </Link>

            </Panel>

            <div className={`${headerStyles.tab_header}`}>
                <div className={`${headerStyles.hamburger_menu}`} onClick={openPanel}>
                <IconButton iconProps={{iconName: 'CollapseMenu'}} title={"menu"}/>
                </div>
            </div>
        </div>
    )
};

export default Header;


/* eslint-disable @typescript-eslint/no-explicit-any */
import { Badge } from 'primereact/badge';
import { Ripple } from "primereact/ripple";
import { classNames } from "primereact/utils";
import React, { KeyboardEvent, useState } from 'react';
import { NavLink, To } from 'react-router-dom';
import { CSSTransition } from 'react-transition-group';

const AppSubmenu = (props: { root?: boolean; parentMenuItemActive?: boolean; onRootMenuitemClick?: (arg0: { originalEvent: any; }) => void; onMenuItemClick?: (arg0?: { originalEvent: any; item: any; }) => void; menuActive?: boolean | null; menuMode?: string; mobileMenuActive?: boolean; items?: any[]; className?: string, role?:string }) => {

    const [activeIndex, setActiveIndex] = useState<number | null>(null)

    const onMenuItemClick = (event: React.MouseEvent<HTMLAnchorElement, MouseEvent>, item: { to?: To; target?: string | undefined; label?: string | undefined; url?: string | undefined; disabled?: any; command?: any; }, index: number) => {
        //avoid processing disabled items
        if (item.disabled) {
            event.preventDefault();
            return true;
        }

        //execute command
        if (item.command) {
            item.command({ originalEvent: event, item: item });
        }

        if (index === activeIndex)
            setActiveIndex(null);
        else
            setActiveIndex(index);

        if (props.onMenuItemClick) {
            props.onMenuItemClick({
                originalEvent: event,
                item: item
            });
        }
    }

    const onKeyDown = (event: KeyboardEvent<HTMLElement>) => {
        if (event.code === 'Enter' || event.code === 'Space') {
            event.preventDefault();
            event.target.click();
        }
    }

    const renderLinkContent = (item: { to?: To; target?: string | undefined; label: any; url?: string | undefined; items?: any; badge?: any; icon?: any; }) => {
        const submenuIcon = item.items && <i className="pi pi-fw pi-angle-down menuitem-toggle-icon"></i>;
        const badge = item.badge && <Badge value={item.badge} />

        return (
            <React.Fragment>
                <i className={item.icon}></i>
                <span>{item.label}</span>
                {submenuIcon}
                {badge}
                <Ripple />
            </React.Fragment>
        );
    }

    const renderLink = (item: { to: To; target: string | undefined; label: string | undefined; url: string | undefined; }, i: number) => {
        const content = renderLinkContent(item);

        if (item.to) {
            return (
                <NavLink className={({ isActive }) => {
                    const linkClasses = ["p-ripple"];
                    if (isActive) linkClasses.push("router-link-active router-link-exact-active");
                    return linkClasses.join(" "); // returns "registerButton" or "registerButton active
                    }} to={item.to} onClick={(e) => onMenuItemClick(e, item, i)} target={item.target}>
                    {content}
                </NavLink>
            )
        }
        else {
            return (
                <a tabIndex={0} aria-label={item.label} onKeyDown={onKeyDown} role="menuitem" href={item.url} className="p-ripple" onClick={(e) => onMenuItemClick(e, item, i)} target={item.target}>
                    {content}
                </a>
            );
        }
    }

    const items = props.items && props.items.map((item, i) => {
        const active = activeIndex === i;
        const styleClass = classNames(item.badgeStyleClass, { 'layout-menuitem-category': props.root, 'active-menuitem': active && !item.to });

        if (props.root) {
            return (
                <li className={styleClass} key={i} role="none">
                    {props.root === true && <React.Fragment>
                        <div className="layout-menuitem-root-text" aria-label={item.label}>{item.label}</div>
                        <AppSubmenu items={item.items} onMenuItemClick={props.onMenuItemClick} />
                    </React.Fragment>}
                </li>
            );
        }
        else {
            return (
                <li className={styleClass} key={i} role="none">
                    {renderLink(item, i)}
                    <CSSTransition classNames="layout-submenu-wrapper" timeout={{ enter: 1000, exit: 450 }} in={active} unmountOnExit>
                        <AppSubmenu items={item.items} onMenuItemClick={props.onMenuItemClick} />
                    </CSSTransition>
                </li>
            );
        }
    });

    return items ? <ul className={props.className} role="menu">{items}</ul> : null;
}

export const AppMenu = (props: { model: any[] | undefined; onMenuItemClick: ((event?: any) => void) | undefined; }) => {

    return (
        <div className="layout-menu-container">
            <AppSubmenu items={props.model} className="layout-menu" onMenuItemClick={props.onMenuItemClick} root={true} role="menu" />
        </div>
    );
}
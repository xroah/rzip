import { ContextMenu } from "@base-ui/react/context-menu"

interface MenuItem {
    name: string
    action?: () => void
}

interface Props {
    menuItems?: MenuItem[]
}

export default function CtxMenu({ menuItems = [] }: Props) {
    return (
        <ContextMenu.Portal>
            <ContextMenu.Positioner sideOffset={4}>
                <ContextMenu.Popup className="zip-ctx-menu rounded-[5px] p-[5px]">
                    {menuItems.map(({ name, action }) => (
                        <ContextMenu.Item
                            key={name}
                            className="zip-ctx-menu-item"
                            onClick={action}
                        >
                            {name}
                        </ContextMenu.Item>
                    ))}
                </ContextMenu.Popup>
            </ContextMenu.Positioner>
        </ContextMenu.Portal>
    )
}

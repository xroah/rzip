import { useEffect, useState, MouseEvent, useRef } from "react"
import { Event, listen, TauriEvent } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/core"
import BackArrow from "./icons/backArrow"
import { type Zip, format } from "./utils/zip"
import { Button } from "@base-ui/react/button"
import { ContextMenu } from "@base-ui/react/context-menu"
import clsx from "clsx"
import CtxMenu from "./CtxMenu"
import Filename from "./Filename"

interface Payload {
    paths: string[]
}

export default function ZipList() {
    const [zip, setZip] = useState<Zip>()
    const [stack, setStack] = useState<Zip[]>([])
    const [selected, setSelected] = useState<Set<number>>(new Set())
    const [editingItems, setEditingItems] = useState<Set<number>>(new Set())
    const [currentCtxMenuItem, setCurrentCtxMenuItem] = useState<
        number | null
    >()
    const rootRef = useRef<HTMLDivElement>(null)
    const [ctxMenuOpen, setCtxMenuOpen] = useState(false)
    const handleCtxMenuOpen = (open: boolean) => {
        setCtxMenuOpen(open)

        if (!open) {
            setCurrentCtxMenuItem(null)
        }
    }
    const removeEditingItems = () => {
        if (editingItems.size) {
            setEditingItems(new Set())
        }
    }
    const handleClick = (e: MouseEvent, idx: number, zip: Zip) => {
        e.stopPropagation()

        if (e.metaKey && e.shiftKey) {
            return
        }

        let id = zip.id

        let newSet = new Set(selected)

        if (e.metaKey) {
            if (newSet.has(id)) {
                newSet.delete(id)
            } else {
                newSet.add(id)
            }
        } else if (e.shiftKey) {
        } else {
            if (newSet.has(id) && newSet.size === 1) {
                newSet = new Set()
            } else {
                newSet = new Set([id])

                removeEditingItems()
            }
        }

        setSelected(newSet)
    }
    const handleDoubleClick = (zip: Zip) => {
        if (zip.isDir) {
            setStack(stack => [...stack, zip.parent!])
            setZip(zip)
            setSelected(new Set())

            return
        }

        setSelected(new Set([zip.id]))
    }
    const handleRootClick = (e: MouseEvent) => {
        let t = e.target as HTMLElement

        // prevent from triggering if context menu items were clicked
        if (rootRef.current == t || rootRef.current?.contains(t)) {
            selected.size && setSelected(new Set())
        }

        removeEditingItems()
    }
    const handleBack = () => {
        let current = stack.pop()

        setZip(current)
        setStack([...stack])
        setSelected(new Set([]))
    }
    const handleRename = (name: string, path: string) => {
        setZip(zip => {
            zip!.children = zip!.children.map(item => {
                if (item.path === path) {
                    item.name = name
                }

                return item
            })

            return { ...zip } as Zip
        })
    }

    useEffect(() => {
        const listenPromise = listen(
            TauriEvent.DRAG_DROP,
            (event: Event<Payload>) => {
                const { paths } = event.payload

                if (paths.length) {
                    invoke("get_zip_json", { file: paths[0] })
                        .then(ret => {
                            const data = JSON.parse(ret as string)

                            format(data, setZip)
                            console.log(data)
                        })
                        .catch(e => {
                            console.error(e)
                        })
                }
            },
        )

        return () => {
            listenPromise.then(rm => rm())
        }
    }, [])

    return (
        <div
            className="zip"
            tabIndex={-1}
            ref={rootRef}
            onClick={handleRootClick}
        >
            <ul className="zip-header">
                <li className="zip-path zip-item">
                    <Button
                        onClick={handleBack}
                        disabled={!stack.length}
                        className="zip-back"
                    >
                        <BackArrow />
                    </Button>
                    <div className="zip-path-details">{zip?.path}</div>
                </li>
                <li className="zip-item">
                    <div className="file-name">Name</div>
                    <div className="file-last-modified">Date modified</div>
                    <div className="file-compressed-size">Compressed size</div>
                    <div className="file-size">Size</div>
                </li>
            </ul>
            <ContextMenu.Root
                open={ctxMenuOpen}
                defaultOpen={false}
                onOpenChange={handleCtxMenuOpen}
            >
                <ul className="zip-list">
                    {zip?.children.map((zip, i) => (
                        <ContextMenu.Trigger
                            key={zip.id}
                            render={
                                <li
                                    key={zip.id}
                                    onClick={e => handleClick(e, i, zip)}
                                    onDoubleClick={() => handleDoubleClick(zip)}
                                    className={clsx("zip-item", {
                                        "zip-item-selected": selected.has(
                                            zip.id,
                                        ),
                                        "zip-item-ctx-menu":
                                            currentCtxMenuItem === zip.id,
                                    })}
                                />
                            }
                            onContextMenu={() => setCurrentCtxMenuItem(zip.id)}
                        >
                            <Filename
                                name={zip.name}
                                icon={zip.icon}
                                path={zip.path}
                                isDir={zip.isDir}
                                onDone={removeEditingItems}
                                onRename={handleRename}
                                isEditing={
                                    editingItems.size === 1 &&
                                    editingItems.has(zip.id)
                                }
                            />
                            <div className="file-last-modified truncate">
                                {zip.modified}
                            </div>
                            <div className="file-compressed-size truncate">
                                {zip.compressedSize}
                            </div>
                            <div className="file-size truncate">{zip.size}</div>
                        </ContextMenu.Trigger>
                    ))}
                </ul>

                <CtxMenu
                    menuItems={[
                        {
                            name: "Open",
                        },
                        {
                            name: "Rename",
                        },
                        {
                            name: "Delete",
                        },
                    ]}
                />
            </ContextMenu.Root>
        </div>
    )
}
